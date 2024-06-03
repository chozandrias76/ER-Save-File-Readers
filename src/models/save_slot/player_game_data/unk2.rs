use std::{
  fmt,
  io::{self, Read, Seek},
  ops::{Deref, DerefMut},
};

use crate::{models::shared::i32_reader::I32Reader, traits::binary_readable::BinaryReadable};

#[derive(serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct Unk2 {
  inner: I32Reader,
}

// Implement Deref trait for Unk2
impl Deref for Unk2 {
  type Target = I32Reader;

  fn deref(&self) -> &Self::Target {
      &self.inner
  }
}

// Implement DerefMut trait for Unk2
impl DerefMut for Unk2 {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.inner
  }
}

// Implement Debug trait for Unk2 by forwarding to I32Reader
impl fmt::Debug for Unk2 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      fmt::Debug::fmt(&self.inner, f)
  }
}

// Implement BinaryReadable trait for Unk2 by forwarding to I32Reader
impl BinaryReadable for Unk2 {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
      Ok(Unk2 {
          inner: I32Reader::read(reader)?,
      })
  }
}
