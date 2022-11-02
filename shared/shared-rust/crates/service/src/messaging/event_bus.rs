use anyhow::Result;
use tokio::sync::Mutex;

use super::{EventBusBackend, EventMetadata, FromEventData, ToEventData, ToEventDataError};

pub struct EventBus {
  backend: Mutex<Box<dyn EventBusBackend + Send>>
}

impl EventBus {
  pub fn new(backend: Box<dyn EventBusBackend + Send>) -> Self {
    Self {
      backend: Mutex::new(backend)
    }
  }

  pub async fn manual_subscribe<'a, Model, Handler, Future>(
    &self,
    id: String,
    metadata: EventMetadata<'a>,
    handler: Handler
  ) where
    Model: FromEventData + Sized + Send + Sync,
    Future: std::future::Future<Output = Result<()>> + Send,
    Handler: Fn(Model) -> Future + Send + Clone + 'static
  {
    self
      .backend
      .lock()
      .await
      .subscribe(
        id,
        metadata,
        Box::new(move |data| {
          let cloned = handler.clone();
          Box::pin(async move {
            let event = Model::from_event_data(&data)?;
            let fut = cloned(event);
            fut.await?;
            Ok(())
          })
        })
      )
      .await
  }

  pub async fn subscribe<Event, Handler, Future>(&self, id: String, handler: Handler)
  where
    Event: super::Event + FromEventData + Sized + Send + Sync,
    Future: std::future::Future<Output = Result<()>> + Sync + Send + 'static,
    Handler: Fn(Event) -> Future + Send + Sync + Clone + 'static
  {
    self.manual_subscribe(id, Event::metadata(), handler).await
  }

  pub async fn manual_publish<'a>(
    &self,
    metadata: EventMetadata<'a>,
    data: impl ToEventData
  ) -> Result<(), ToEventDataError> {
    let data = data.to_event_data()?;

    let lock_fut = self.backend.lock();
    let mut backend = lock_fut.await;
    let publish_fut = backend.publish(metadata, &data);
    publish_fut.await;

    Ok(())
  }

  pub async fn publish<Event>(&self, event: Event) -> Result<(), ToEventDataError>
  where
    Event: super::Event + ToEventData
  {
    self.manual_publish(Event::metadata(), event).await
  }
}
