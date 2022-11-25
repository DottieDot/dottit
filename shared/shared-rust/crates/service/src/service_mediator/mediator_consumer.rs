use anyhow::Result;
use std::{future::Future, sync::Arc};

use crate::{
  events::{self, MediatorResponse, MEDIATOR_EXCHANGE},
  messaging::{EventBus, EventMetadata, FromEventData, QueueMetadata, QueueOptions, ToEventData}
};

use super::MediatorQuery;

pub struct MediatorConsumer {
  event_bus: Arc<EventBus>
}

impl MediatorConsumer {
  pub fn new(event_bus: Arc<EventBus>) -> Self {
    Self { event_bus }
  }

  pub async fn subscribe<Q, Fut>(&self, handler: impl Fn(Q) -> Fut + Send + Clone + 'static)
  where
    Q: MediatorQuery + FromEventData + Send,
    Q::Response: ToEventData + Send,
    Fut: Future<Output = Result<Q::Response>> + Send
  {
    let event_bus = self.event_bus.clone();

    self
      .event_bus
      .manual_subscribe(
        Q::name().to_owned(),
        EventMetadata {
          exchange:        MEDIATOR_EXCHANGE,
          queue:           QueueMetadata {
            routing_key:  Q::name(),
            options:      QueueOptions {
              exclusive: true,
              ..Default::default()
            },
            bind_options: Default::default()
          },
          consume_options: Default::default()
        },
        move |data: events::MediatorQuery| {
          let cloned_handler = handler.clone();
          let event_bus = event_bus.clone();
          async move {
            let event_data = Q::from_event_data(&data.data)?;
            let fut = cloned_handler(event_data);
            let result = fut.await?;

            let response = MediatorResponse {
              query_id: data.query_id,
              data:     result.to_event_data()?
            };

            event_bus
              .manual_publish(
                EventMetadata {
                  exchange:        MEDIATOR_EXCHANGE,
                  queue:           QueueMetadata {
                    routing_key:  &data.return_address,
                    options:      Default::default(),
                    bind_options: Default::default()
                  },
                  consume_options: Default::default()
                },
                response
              )
              .await?;

            Ok(())
          }
        }
      )
      .await;
  }
}
