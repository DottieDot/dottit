use crate::messaging::{EventExchangeType, ExchangeMetadata, ExchangeOptions};

pub(crate) const THREAD_EXCHANGE: ExchangeMetadata = ExchangeMetadata {
  name:          "thread_service",
  exchange_type: EventExchangeType::Direct,
  options:       ExchangeOptions {
    passive:     false,
    durable:     true,
    auto_delete: false,
    internal:    false,
    no_wait:     false
  }
};

pub const MEDIATOR_EXCHANGE: ExchangeMetadata = ExchangeMetadata {
  name:          "mediator",
  exchange_type: EventExchangeType::Direct,
  options:       ExchangeOptions {
    passive:     false,
    durable:     false,
    auto_delete: false,
    internal:    false,
    no_wait:     false
  }
};
