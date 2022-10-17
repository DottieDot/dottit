use binary_reader::BinaryReader;

use crate::messaging::{FromEventData, FromEventDataError};

pub struct MediatorQuery {
  pub return_address: String,
  pub data:           Vec<u8>
}

impl FromEventData for MediatorQuery {
  fn from_event_data(data: &[u8]) -> Result<Self, FromEventDataError> {
    let mut reader = BinaryReader::from_u8(data);

    let return_address = reader.read_cstr().map_err(|e| {
      FromEventDataError {
        msg:    "failed to read cstring from event data".to_owned(),
        source: Some(e.into())
      }
    })?;

    let data = reader.data[reader.pos..].to_owned();

    Ok(Self {
      return_address,
      data
    })
  }
}
