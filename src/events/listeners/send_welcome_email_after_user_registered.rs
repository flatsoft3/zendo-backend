use crate::common::services::email::email_service::SendEmailStatus;
use crate::state::AppState;

pub async fn listen_to_user_registered_event(state: AppState) {
    let mut rx = state.events_bus.user_registered_event_bus.subscribe();
    while let Ok(event) = rx.recv().await {

        tracing::info!("Sending welcome email for new signup");

        match state.common_services.email.send_welcome_mail(&event.user.email, &event.user.get_full_name(), &state.config.app_frontend_users_url)
        .await
        {
            Ok(send_email_status) => match send_email_status {
                SendEmailStatus::Sent =>
                tracing::info!("New signup email sent to user"),

                x => 
                 tracing::error!("{}", format!("Error while sending signup email, {:#?}", x))
            }
            
            Err(e) => {
                tracing::error!("{}", format!("Failed to send new signup email, {:#?}", e))
            }
        }
    }
}
