use std::io;
use std::{io::{Read, Seek}, ops::{Deref, DerefMut}};

use crate::traits::binary_readable::BinaryReadable;

use super::model::Model;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Eyelash {
    pub model: Model,
}

impl Default for Eyelash {
    fn default() -> Self {
        Eyelash {
            model: Model::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Model
impl Deref for Eyelash {
    type Target = Model;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl DerefMut for Eyelash {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.model
    }
}

impl BinaryReadable for Eyelash {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
    Ok(Eyelash {
      model: Model::read(reader)?,
    })
  }
}