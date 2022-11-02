use binary_reader::BinaryReader;

use crate::messaging::{FromEventData, FromEventDataError, ToEventData};

#[derive(Clone)]
pub struct MediatorResponse {
  pub query_id: String,
  pub data:     Vec<u8>
}

impl From<MediatorResponse> for Vec<u8> {
  fn from(data: MediatorResponse) -> Self {
    [data.query_id.into_bytes(), vec![0], data.data].concat()
  }
}

impl ToEventData for MediatorResponse {
  fn to_event_data(&self) -> Result<Vec<u8>, crate::messaging::ToEventDataError> {
    Ok(self.clone().into())
  }
}

impl FromEventData for MediatorResponse {
  fn from_event_data(data: &[u8]) -> Result<Self, FromEventDataError> {
    let mut reader = BinaryReader::from_u8(data);

    let query_id = reader.read_cstr().map_err(|e| {
      FromEventDataError {
        msg:    "failed to read query id from event data".to_owned(),
        source: Some(e.into())
      }
    })?;

    let data = reader.data[reader.pos..].to_owned();

    Ok(MediatorResponse { query_id, data })
  }
}
