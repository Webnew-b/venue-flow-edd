use std::sync::Arc;

use app::app_event::AppEvent;

use crate::event_error::{EventError, EventResult};
use crate::event_service::email_service::{
    generate_cancellation_email_simple,
    generate_cancellation_email_simple_for_organizer,
    generate_rental_email_simple, EmailService, EmailStruct,
};

pub struct EventConsumer {
    email_service: Arc<EmailService>,
}

impl EventConsumer {
    pub fn new(email_service: Arc<EmailService>) -> Self {
        Self { email_service }
    }
    pub async fn execute(&self, event: AppEvent) -> EventResult<()> {
        match event {
            AppEvent::CanceledRentalRequest {
                organizer_email,
                organizer_name,
                organizer_id,
                lessor_id,
                lessor_name,
                lessor_email,
            } => {
                let organizer_content =
                    generate_cancellation_email_simple_for_organizer(
                        organizer_name.as_str(),
                        lessor_name.as_str(),
                        "test",
                    );
                let organizer_email = EmailStruct {
                    from:    self.email_service.sender.clone(),
                    to:      organizer_email,
                    content: organizer_content,
                };

                let lessor_content = generate_cancellation_email_simple(
                    organizer_name.as_str(),
                    lessor_name.as_str(),
                    "test",
                );
                let lessor_email = EmailStruct {
                    from:    self.email_service.sender.clone(),
                    to:      lessor_email,
                    content: lessor_content,
                };

                self.email_service.send_email(organizer_email).await?;
                self.email_service.send_email(lessor_email).await?;

                tracing::info!(
                    "Sended CancelRentalRequest Email to lessor {lessor_id} and organizer {organizer_id}");

                Ok(())
            },
            AppEvent::ApprovedRentalRequest {
                organizer_email,
                organizer_name,
                organizer_id,
            } => {
                let organizer_content = generate_rental_email_simple(
                    organizer_name.as_str(),
                    "test",
                    "同意租赁",
                );
                let organizer_email = EmailStruct {
                    from:    self.email_service.sender.clone(),
                    to:      organizer_email,
                    content: organizer_content,
                };
                self.email_service.send_email(organizer_email).await?;
                tracing::info!(
                    "Sended ApprovedRentalRequest Email to organizer {}",
                    organizer_id
                );
                Ok(())
            },
            AppEvent::RejectedRentalRequest {
                organizer_email,
                organizer_name,
                organizer_id,
            } => {
                let organizer_content = generate_rental_email_simple(
                    organizer_name.as_str(),
                    "test",
                    "拒绝租赁",
                );
                let organizer_email = EmailStruct {
                    from:    self.email_service.sender.clone(),
                    to:      organizer_email,
                    content: organizer_content,
                };
                self.email_service.send_email(organizer_email).await?;
                tracing::info!(
                    "Sended RejectedRentalRequest Email to organizer {}",
                    organizer_id
                );
                Ok(())
            },
            _ => Err(EventError::WrongExecutionMode),
        }
    }
}
