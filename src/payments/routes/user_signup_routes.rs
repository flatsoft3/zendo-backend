use crate::common::util;
use crate::dtos::requests::CreateUserRequest;
use crate::dtos::responses::UserCreatedResponse;
use crate::events::user_registered::UserRegisteredEvent;
use crate::models::user::User;
use crate::{
    common::{error::AppError, structs::ApiResponse},
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use rand::Rng;
use redis::AsyncCommands;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use axum::response::Result;

#[derive(Deserialize, Validate)]
pub struct RequestEmailCodeQuery {
    #[validate(email)]
    pub email: String,
}

pub async fn request_email_verification_code(
    State(state): State<AppState>,
    Json(payload): Json<RequestEmailCodeQuery>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::validation_error(e))?;

    let email = payload.email;

    match User::find_by_email(&state.db_pool, &email).await {
        Err(e) => Err(e.into()),
        Ok(Some(_)) => Err(AppError::bad_request("Email already exists")),
        Ok(None) => {
            //generate the code
            let code = {
                let mut rng = rand::thread_rng();
                rng.gen_range(100_000u64..999_999u64)
            };

            //send the code to the email
            state
                .common_services
                .email
                .send_email_verification_code(&email, &code.to_string())
                .await?;

            //save the code to redis
            let mut connection = state
                .redis_client
                .get_multiplexed_async_connection()
                .await?;
            let _: () = connection
                .set_ex(
                    format!("zendo_email_validation_code:{}", &email),
                    code,
                    1800,
                )
                .await?;

            //return success

            let response: ApiResponse<serde_json::Value> =
                ApiResponse::success("Verification code sent to your email", None);

            return Ok((StatusCode::OK, Json(response)));
        }
    }
}

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::validation_error(e))?;

    //validate the code
    let mut connection = state
        .redis_client
        .get_multiplexed_async_connection()
        .await?;

    let result: Option<String> = connection
        .get(format!("zendo_email_validation_code:{}", &payload.email))
        .await?;

    // if result.is_none()
    //     || result
    //         .as_ref()
    //         .is_some_and(|code| code != &payload.email_verification_code)
    // {
    //     return Err(AppError::bad_request(
    //         "Invalid or expired verification code",
    //     ));
    // }

    let Some(code) = result else {
        return Err(AppError::bad_request("Email verification code has already expired"));
    };

    if code != payload.email_verification_code {
        return Err(AppError::bad_request("Invalid email verification code"));
    }

    match User::find_by_email(&state.db_pool, &payload.email).await {
        Err(e) => Err(e.into()),
        Ok(Some(_)) => Err(AppError::bad_request("User already exists")),
        Ok(None) => {
            match User::create(
                &state.db_pool,
                Uuid::new_v4(),
                &payload.email,
                &payload.first_name,
                payload.middle_name.as_deref(),
                &payload.last_name,
                &payload.phone_number,
                &util::hash_password(&payload.password),
                None,
            )
            .await
            {
                Ok(new_user) => {
                    let response: ApiResponse<UserCreatedResponse> = ApiResponse::success(
                        "User was created successfully",
                        Some(new_user.clone().into()),
                    );

                    //publish user created event
                    state
                        .events_bus
                        .user_registered_event_bus
                        .publish(UserRegisteredEvent { user: new_user });

                    Ok((StatusCode::CREATED, Json(response)))
                }
                Err(e) => Err(e),
            }
        }
    }
}
