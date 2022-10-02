use super::event_metadata::EventMetadata;

pub trait Event {
  fn metadata() -> EventMetadata;
}
