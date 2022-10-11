use anyhow::Result;
use std::future::Future;

use super::MediatorQuery;

pub struct MediatorConsumer {
  // event_bus: Arc<EventBus>
}

impl MediatorConsumer {
  pub async fn subscribe<Q, Fut>(&self, _: impl Fn(Q) -> Fut)
  where
    Q: MediatorQuery + Default,
    Fut: Future<Output = Result<()>> + Sync + Send + 'static
  {
    // self
    //   .event_bus
    //   .subscribe(Q::name().to_owned(), |_: Vec<u8>| handler(Q::default()))
    //   .await;
    todo!()
  }
}
