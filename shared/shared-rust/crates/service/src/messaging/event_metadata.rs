#[derive(Clone, Copy, Debug)]
pub struct EventMetadata<'a> {
  pub exchange:        ExchangeMetadata<'a>,
  pub queue:           QueueMetadata<'a>,
  pub consume_options: ConsumeOptions
}

#[derive(Clone, Copy, Debug)]
pub struct ExchangeMetadata<'a> {
  pub name:          &'a str,
  pub exchange_type: EventExchangeType,
  pub options:       ExchangeOptions
}

#[derive(Clone, Copy, Debug)]
pub enum EventExchangeType {
  Direct,
  Topic,
  Headers,
  FanOut
}

#[derive(Default, Clone, Copy, Debug)]
pub struct ExchangeOptions {
  pub passive:     bool,
  pub durable:     bool,
  pub auto_delete: bool,
  pub internal:    bool,
  pub no_wait:     bool
}

#[derive(Clone, Copy, Debug)]
pub struct QueueMetadata<'a> {
  pub routing_key:  &'a str,
  pub options:      QueueOptions,
  pub bind_options: QueueBindOptions
}

#[derive(Default, Clone, Copy, Debug)]
pub struct QueueOptions {
  pub passive:     bool,
  pub durable:     bool,
  pub exclusive:   bool,
  pub auto_delete: bool,
  pub no_wait:     bool
}

#[derive(Default, Clone, Copy, Debug)]
pub struct QueueBindOptions {
  pub no_wait: bool
}

#[derive(Default, Clone, Copy, Debug)]
pub struct ConsumeOptions {
  pub no_local:  bool,
  pub no_ack:    bool,
  pub exclusive: bool,
  pub no_wait:   bool
}
