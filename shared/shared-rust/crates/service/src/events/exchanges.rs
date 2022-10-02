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
