use std::{
  fmt,
  io::{self, Read, Seek},
};

use crate::traits::{binary_readable::BinaryReadable, validate::Validate};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct BoolReader {
  pub data: bool,
}

impl Default for BoolReader {
  fn default() -> Self {
      Self {
          data: Default::default(),
      }
  }
}

impl fmt::Debug for BoolReader {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "BoolReader(\n")?;
      write!(f, "{}", self.data.to_string())?;
      write!(f, ")")
  }
}

impl BinaryReadable for BoolReader {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
      let mut mutable_default = [0u8; 1];
      reader
          .read_exact(&mut mutable_default)
          .expect("data should be present");
      Ok(BoolReader {
          data: bool::from(mutable_default[0] != 0), // Convert the [u8] slice to bool
      })
  }
}

impl Validate for BoolReader {
  fn validate(&self) -> bool {
      let possible_values: [u8; 2] = [0,1];
      possible_values.iter().find(|i| **i == u8::from(self.data)).is_some()
  }
}
