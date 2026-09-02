use crate::common::services::email::email_service::{EmailPayload, SendEmailStatus};
use crate::common::services::email::email_templates::WelcomeEmail; 
use crate::state::AppState;
use askama::Template; 

pub async fn listen_to_user_registered_event(state: AppState) {
    let mut rx = state.events_bus.user_registered_event_bus.subscribe();
    while let Ok(event) = rx.recv().await {

        tracing::info!("Sending welcome email for new signup");

        let email = WelcomeEmail {
            name: &event.user.get_full_name() ,
            verify_email_url: "https://yourapp.com/start",
        };

        let raw_html = email.render().unwrap(); 

        let payload = EmailPayload {
            to: event.user.email,
            subject: "New Signup - Verify your email".into(),
            body: css_inline::inline(&raw_html).unwrap()
        };

        match state.common_services.email.send_mail(payload)
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
