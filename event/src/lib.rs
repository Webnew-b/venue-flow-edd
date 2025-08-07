use std::sync::Arc;

use app::Outcome;
use tokio::task::JoinHandle;

use crate::consumer::EventConsumer;
use crate::dispatcher::EventDispatcher;
use crate::event_error::EventResult;
use crate::event_service::email_service::EmailService;
use crate::poller::{EventPoller, PollerConfig};
use crate::queue::AsyncQueue;

pub mod dispatcher;
pub mod queue;
pub mod consumer;
pub mod event_error;
pub mod poller;
pub mod event_service;

pub struct EventSystem {
    queue:Arc<dyn AsyncQueue>,
   dispatcher: EventDispatcher,
   poller: EventPoller,
   _poller_handle: JoinHandle<()>,
}

impl EventSystem {
   pub async fn new(
       async_queue: Arc<dyn AsyncQueue>,
       email_service: Arc<EmailService>,
   ) -> EventResult<Self> {
       let consumer = Arc::new(EventConsumer::new(
           email_service,
       ));
       
       let dispatcher = EventDispatcher;
       
       let poller = EventPoller::new(
           async_queue.clone(),
           consumer,
           PollerConfig::default(),
       );
       
       let poller_handle = {
           let poller_clone = poller.clone();
           tokio::spawn(async move {
               if let Err(e) = poller_clone.start_polling().await {
                   tracing::error!("Event poller error: {:?}", e);
               }
           })
       };
       
       Ok(EventSystem {
           queue:async_queue,
           dispatcher,
           poller,
           _poller_handle: poller_handle,
       })
   }
   
   /// 主要API：处理usecase的输出 - 立即事件执行，异步事件入队
   pub async fn process_outcome<T>(&self, outcome: Outcome<T>) -> EventResult<T> {
       self.dispatcher.process_outcome(outcome,&*self.queue).await
   }
   
   /// 优雅关闭系统
   pub fn shutdown(&self) {
       self.poller.shutdown();
   }
}


