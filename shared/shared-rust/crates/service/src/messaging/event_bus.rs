use anyhow::Result;
use tokio::sync::Mutex;

use super::{EventBusBackend, FromEventData, IntoEventData};

pub struct EventBus {
  backend: Mutex<dyn EventBusBackend>
}

impl EventBus {
  pub async fn subscribe<Event, Handler, Future>(&self, id: String, handler: Handler)
  where
    Event: super::Event + FromEventData + Sized + Send + Sync,
    Future: std::future::Future<Output = Result<()>> + Sync + Send + 'static,
    Handler: Fn(Event) -> Future + Send + Sync + Clone + 'static
  {
    self
      .backend
      .lock()
      .await
      .subscribe(
        id,
        Event::metadata(),
        Box::new(move |data| {
          let cloned = handler.clone();
          Box::pin(async move {
            let event = Event::from_event_data(&data)?;
            cloned(event).await?;
            Ok(())
          })
        })
      )
      .await;
  }

  pub async fn publish<Event>(&self, event: Event)
  where
    Event: super::Event + IntoEventData
  {
    self
      .backend
      .lock()
      .await
      .publish(Event::metadata(), &event.into_event_data().unwrap())
      .await;
  }
}
