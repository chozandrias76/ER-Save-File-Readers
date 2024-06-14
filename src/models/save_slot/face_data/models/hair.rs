use std::io;
use std::{io::{Read, Seek}, ops::{Deref, DerefMut}};

use crate::models::shared::byte_array_reader::ByteArrayReadable;
use crate::traits::binary_readable::BinaryReadable;

use super::model::Model;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Hair {
    pub model: Model,
}

impl Default for Hair {
    fn default() -> Self {
        Hair {
            model: Model::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Model
impl Deref for Hair {
    type Target = Model;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl DerefMut for Hair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.model
    }
}

impl BinaryReadable for Hair {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
    Ok(Hair {
      model: Model::read(reader)?,
    })
  }
}