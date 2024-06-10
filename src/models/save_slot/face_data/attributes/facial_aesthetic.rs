use std::ops::{Deref, DerefMut};
use super::attribute::Attribute;
use std::{
  io::{self, Read, Seek},
  ops::{Deref, DerefMut},
};

pub struct FacialAesthetic {
  pub attribute: Attribute,
}

impl Default for FacialAesthetic {
  fn default() -> Self {
      FacialAesthetic {
          attribute: Attribute::default(),
      }
  }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for FacialAesthetic {
  type Target = Attribute;

  fn deref(&self) -> &Self::Target {
      &self.attribute
  }
}

impl DerefMut for FacialAesthetic {
  fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.attribute
  }
}

impl BinaryReadable for FacialAesthetic {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
      Ok(FacialAesthetic {
          attribute: Attribute::read(reader)?,
      })
  }
}