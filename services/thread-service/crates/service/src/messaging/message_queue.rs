use std::future::Future;

use tokio::sync::Mutex;

use super::message_queue_backend::MessageQueueBackend;

pub struct MessageQueue {
  backend: Mutex<Box<dyn MessageQueueBackend + Send + 'static>>
}

impl MessageQueue {
  pub fn new(backend: Mutex<Box<dyn MessageQueueBackend + Send + 'static>>) -> Self {
    Self { backend }
  }
}

impl MessageQueue {
  pub async fn publish<'a, T: Into<&'a [u8]>>(&'a self, data: T) {
    self.backend.lock().await.publish(data.into()).await
  }

  pub async fn subscribe<T, Fut>(
    &self,
    consumer_tag: &str,
    handler: impl Fn(T) -> Fut + Send + Sync + Clone + 'static
  ) where
    T: TryFrom<Vec<u8>> + Send + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
    <T as TryFrom<Vec<u8>>>::Error: Send
  {
    self
      .backend
      .lock()
      .await
      .subscribe(
        consumer_tag,
        Box::new(move |bytes| {
          let cloned = handler.clone();
          Box::pin(async move {
            if let Ok(data) = T::try_from(bytes) {
              cloned(data).await
            }
          })
        })
      )
      .await
  }
}
