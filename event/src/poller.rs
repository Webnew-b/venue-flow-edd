use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio::time::{interval, Duration};

use crate::consumer::EventConsumer;
use crate::event_error::EventResult;
use crate::queue::AsyncQueue;

#[derive(Clone)]
pub struct PollerConfig {
    pub poll_interval:Duration,
    pub batch_size:usize,
    pub max_retries:u32,
    pub retry_delay: Duration,
}

pub struct EventPoller {
    queue: Arc<dyn AsyncQueue>,
    consumer: Arc<EventConsumer>,
    config: PollerConfig,
    shutdown_signal: Arc<AtomicBool>,
}


impl Default for PollerConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(5),
            batch_size: 10,
            max_retries: 3,
            retry_delay: Duration::from_secs(30),
        }
    }
}


impl EventPoller {
    pub fn new(
        queue: Arc<dyn AsyncQueue>,
        consumer: Arc<EventConsumer>,
        config: PollerConfig,
    ) -> Self {
        Self {
            queue,
            consumer,
            config,
            shutdown_signal: Arc::new(AtomicBool::new(false)),
        }
    }
    
    pub async fn start_polling(&self) -> EventResult<()> {
        let mut interval = interval(self.config.poll_interval);
        
        while !self.shutdown_signal.load(Ordering::Relaxed) {
            interval.tick().await;
            
            match self.poll_and_process_batch().await {
                Ok(processed_count) => {
                    if processed_count > 0 {
                        tracing::debug!("Processed {} async events", processed_count);
                    }
                }
                Err(e) => {
                    tracing::error!("Event polling error: {:?}", e);
                }
            }
        }
        
        Ok(())
    }
    
    async fn poll_and_process_batch(&self) -> EventResult<usize> {
        let events = self.queue.pop_batch(self.config.batch_size).await?;
        let mut processed = 0;
        
        for event in events {
            match self.consumer.execute(event.event.clone()).await {
                Ok(_) => {
                    self.queue.mark_processed(event.id).await?;
                    processed += 1;
                }
                Err(e) => {
                    let error_msg = format!("{:#}", e);
                    self.queue.mark_failed(event.id, error_msg).await?;
                    tracing::warn!("Event execution failed: {:?}", e);
                }
            }
        }
        
        Ok(processed)
    }
    
    pub fn shutdown(&self) {
        self.shutdown_signal.store(true, Ordering::Relaxed);
    }
}

impl Clone for EventPoller {
   fn clone(&self) -> Self {
       Self {
           queue: self.queue.clone(),
           consumer: self.consumer.clone(),
           config: self.config.clone(),
           shutdown_signal: self.shutdown_signal.clone(),
       }
   }
}
