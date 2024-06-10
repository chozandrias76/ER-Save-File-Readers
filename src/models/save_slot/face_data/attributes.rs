use std::{io::{self, Read, Seek}, ops::{Deref, DerefMut}};

use crate::{models::shared::u8_reader::U8Reader, traits::binary_readable::BinaryReadable};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Attribute {
    pub data: u8,
}

// Implement BinaryReadable trait for Attribute by forwarding to U8Reader
impl BinaryReadable for Attribute {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
      Ok(Attribute {
          data: U8Reader::read(reader)?.data,
      })
  }
}

impl Default for Attribute {
    fn default() -> Self {
        Attribute { data: U8Reader::default().data }
    }
}

pub struct ApparentAge {
    pub attribute: Attribute,
}

impl Default for ApparentAge {
    fn default() -> Self {
        ApparentAge {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for ApparentAge {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for ApparentAge {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}
