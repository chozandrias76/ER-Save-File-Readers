use std::io;
use std::{io::{Read, Seek}, ops::{Deref, DerefMut}};

use crate::models::shared::byte_array_reader::ByteArrayReadable;
use crate::traits::binary_readable::BinaryReadable;

use super::model::Model;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Accessory {
    pub model: Model,
}

impl Default for Accessory {
    fn default() -> Self {
        Accessory {
            model: Model::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Model
impl Deref for Accessory {
    type Target = Model;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl DerefMut for Accessory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.model
    }
}

impl BinaryReadable for Accessory {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
    Ok(Accessory {
      model: Model::read(reader)?,
    })
  }
}