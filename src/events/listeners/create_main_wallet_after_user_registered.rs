use crate::common::enums::Currency;
use crate::models::wallet::Wallet;
use crate::state::AppState;
use uuid::Uuid;

pub async fn listen_to_user_registered_event(state: AppState) {
    let mut rx = state.events_bus.user_registered_event_bus.subscribe();
    while let Ok(event) = rx.recv().await {
        match Wallet::create(
            &state.db_pool,
            Uuid::new_v4(),
            &event.user.id,
            "Main",
            Currency::NGN,
        )
        .await
        {
            Ok(_) => println!("created user main wallet"),
            Err(e) => {
                println!("{}", format!("Failed to create user wallet, {:#?}", e))
            }
        }
    }
}
