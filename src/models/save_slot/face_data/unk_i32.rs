use std::{
  fmt,
  io::{self, Read, Seek},
  ops::{Deref, DerefMut},
};

use crate::{
  models::shared::i32_reader::I32Reader,
  traits::{binary_readable::BinaryReadable, validate::Validate},
};

#[derive(serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct UnkI32 {
  inner: I32Reader,
}

// Implement Deref trait for UnkI32
impl Deref for UnkI32 {
  type Target = I32Reader;

  fn deref(&self) -> &Self::Target {
      &self.inner
  }
}

// Implement DerefMut trait for UnkI32
impl DerefMut for UnkI32 {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.inner
  }
}

// Implement Debug trait for UnkI32 by forwarding to I32Reader
impl fmt::Debug for UnkI32 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      fmt::Debug::fmt(&self.inner, f)
  }
}

// Implement BinaryReadable trait for UnkI32 by forwarding to I32Reader
impl BinaryReadable for UnkI32 {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
      Ok(UnkI32 {
          inner: I32Reader::read(reader)?,
      })
  }
}

impl Validate for UnkI32 {
  fn validate(&self) -> bool {
      self.inner.validate()
  }
}
