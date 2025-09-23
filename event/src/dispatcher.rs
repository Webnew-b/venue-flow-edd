use app::app_event::AppEvent;
use app::{AppUseCase, Outcome};
use domain::event_trait::EventExecutionMode;

use crate::event_error::{EventError, EventResult};
use crate::queue::{AsyncQueue, QueueEvent};

pub struct EventDispatcher;

impl EventDispatcher {
    pub async fn process_outcome<T>(
        &self,
        outcome: Outcome<T>,
        async_queue: &dyn AsyncQueue,
    ) -> EventResult<T> {
        for event in outcome.events.get_elements() {
            match event.execution_mode() {
                EventExecutionMode::Immediate => {
                    self.execute_immediately(event, outcome.from_case.clone())?;
                },
                EventExecutionMode::Async => {
                    let event = QueueEvent::new(event);
                    async_queue.push(event).await?;
                },
            }
        }
        Ok(outcome.data)
    }

    pub fn execute_immediately(
        &self,
        event: AppEvent,
        use_case: AppUseCase,
    ) -> EventResult<()> {
        match event {
            AppEvent::LogUseCase => {
                tracing::info!("The use case:{use_case} executed.");
                Ok(())
            },
            _ => Err(EventError::WrongExecutionMode),
        }
    }
}
