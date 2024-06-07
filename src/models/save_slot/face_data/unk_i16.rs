use std::{
  fmt,
  io::{self, Read, Seek},
  ops::{Deref, DerefMut},
};

use crate::{
  models::shared::i16_reader::I16Reader,
  traits::{binary_readable::BinaryReadable, validate::Validate},
};

#[derive(serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct UnkI16 {
  inner: I16Reader,
}

// Implement Deref trait for UnkI16
impl Deref for UnkI16 {
  type Target = I16Reader;

  fn deref(&self) -> &Self::Target {
      &self.inner
  }
}

// Implement DerefMut trait for UnkI16
impl DerefMut for UnkI16 {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.inner
  }
}

// Implement Debug trait for UnkI16 by forwarding to I16Reader
impl fmt::Debug for UnkI16 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      fmt::Debug::fmt(&self.inner, f)
  }
}

// Implement BinaryReadable trait for UnkI16 by forwarding to I16Reader
impl BinaryReadable for UnkI16 {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
      Ok(UnkI16 {
          inner: I16Reader::read(reader)?,
      })
  }
}

impl Validate for UnkI16 {
  fn validate(&self) -> bool {
      self.inner.validate()
  }
}
