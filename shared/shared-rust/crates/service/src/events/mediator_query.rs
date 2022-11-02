use binary_reader::BinaryReader;

use crate::messaging::{FromEventData, FromEventDataError, ToEventData};

#[derive(Clone)]
pub struct MediatorQuery {
  pub return_address: String,
  pub query_id:       String,
  pub data:           Vec<u8>
}

impl From<MediatorQuery> for Vec<u8> {
  fn from(query: MediatorQuery) -> Self {
    [
      query.return_address.into_bytes(),
      vec![0],
      query.query_id.into_bytes(),
      vec![0],
      query.data
    ]
    .concat()
  }
}

impl ToEventData for MediatorQuery {
  fn to_event_data(&self) -> Result<Vec<u8>, crate::messaging::ToEventDataError> {
    Ok(Vec::<u8>::from(self.clone()))
  }
}

impl FromEventData for MediatorQuery {
  fn from_event_data(data: &[u8]) -> Result<Self, FromEventDataError> {
    let mut reader = BinaryReader::from_u8(data);

    let return_address = reader.read_cstr().map_err(|e| {
      FromEventDataError {
        msg:    "failed to read return address from event data".to_owned(),
        source: Some(e.into())
      }
    })?;

    let query_id = reader.read_cstr().map_err(|e| {
      FromEventDataError {
        msg:    "failed to read query id from event data".to_owned(),
        source: Some(e.into())
      }
    })?;

    let data = reader.data[reader.pos..].to_owned();

    Ok(Self {
      return_address,
      query_id,
      data
    })
  }
}
