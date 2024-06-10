use std::io;
use std::{io::{Read, Seek}, ops::{Deref, DerefMut}};

use crate::traits::binary_readable::BinaryReadable;

use super::Model;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Beard {
    pub model: Model,
}

impl Default for Beard {
    fn default() -> Self {
        Beard {
            model: Model::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Model
impl Deref for Beard {
    type Target = Model;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl DerefMut for Beard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.model
    }
}

impl BinaryReadable for Beard {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
    Ok(Beard {
      model: Model::read(reader)?,
    })
  }
}