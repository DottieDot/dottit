use std::{env, sync::Arc};

use shared_infra::messaging::AmqpEventBusBackend;
use shared_service::messaging::EventBus;

pub async fn connect() -> Arc<EventBus> {
  let rmq_host = env::var("RABBITMQ_HOST").expect("no rmq host provided");
  let rmq_user = env::var("RABBITMQ_USER").expect("no rmq user provided");
  let rmq_pass = env::var("RABBITMQ_PASSWORD").expect("no rmq password provided");

  let uri = format!("amqp://{rmq_user}:{rmq_pass}@{rmq_host}");

  println!("{uri}");

  let rmq = AmqpEventBusBackend::connect(&uri).await;

  Arc::new(EventBus::new(Box::new(rmq)))
}
