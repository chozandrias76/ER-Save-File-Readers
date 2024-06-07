use std::{
  fmt,
  io::{self, Read, Seek},
  ops::{Deref, DerefMut},
};

use crate::{
  models::shared::i64_reader::I64Reader,
  traits::{binary_readable::BinaryReadable, validate::Validate},
};

#[derive(serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct UnkI64 {
  inner: I64Reader,
}

// Implement Deref trait for UnkI64
impl Deref for UnkI64 {
  type Target = I64Reader;

  fn deref(&self) -> &Self::Target {
      &self.inner
  }
}

// Implement DerefMut trait for UnkI64
impl DerefMut for UnkI64 {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.inner
  }
}

// Implement Debug trait for UnkI64 by forwarding to I64Reader
impl fmt::Debug for UnkI64 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      fmt::Debug::fmt(&self.inner, f)
  }
}

// Implement BinaryReadable trait for UnkI64 by forwarding to I64Reader
impl BinaryReadable for UnkI64 {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
      Ok(UnkI64 {
          inner: I64Reader::read(reader)?,
      })
  }
}

impl Validate for UnkI64 {
  fn validate(&self) -> bool {
      self.inner.validate()
  }
}
