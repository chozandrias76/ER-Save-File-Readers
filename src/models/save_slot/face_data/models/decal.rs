use std::io;
use std::{io::{Read, Seek}, ops::{Deref, DerefMut}};

use crate::traits::byte_array_readable::ByteArrayReadable;
use crate::traits::binary_readable::BinaryReadable;

use super::model::Model;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Decal {
    pub model: Model,
}

impl Default for Decal {
    fn default() -> Self {
        Decal {
            model: Model::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Model
impl Deref for Decal {
    type Target = Model;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl DerefMut for Decal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.model
    }
}

impl BinaryReadable for Decal {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
    Ok(Decal {
      model: Model::read(reader)?,
    })
  }
}