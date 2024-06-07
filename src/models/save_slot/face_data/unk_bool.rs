use std::{
  fmt,
  io::{self, Read, Seek},
  ops::{Deref, DerefMut},
};

use crate::{
  models::shared::bool_reader::BoolReader,
  traits::{binary_readable::BinaryReadable, validate::Validate},
};

#[derive(serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct UnkBool {
  inner: BoolReader,
}

// Implement Deref trait for UnkBool
impl Deref for UnkBool {
  type Target = BoolReader;

  fn deref(&self) -> &Self::Target {
      &self.inner
  }
}

// Implement DerefMut trait for UnkBool
impl DerefMut for UnkBool {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.inner
  }
}

// Implement Debug trait for UnkBool by forwarding to BoolReader
impl fmt::Debug for UnkBool {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      fmt::Debug::fmt(&self.inner, f)
  }
}

// Implement BinaryReadable trait for UnkBool by forwarding to BoolReader
impl BinaryReadable for UnkBool {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
      Ok(UnkBool {
          inner: BoolReader::read(reader)?,
      })
  }
}

impl Validate for UnkBool {
  fn validate(&self) -> bool {
      self.inner.validate()
  }
}
