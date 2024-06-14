use std::io;
use std::{io::{Read, Seek}, ops::{Deref, DerefMut}};

use crate::traits::binary_readable::BinaryReadable;

use super::model::Model;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Eyebrow {
    pub model: Model,
}

impl Default for Eyebrow {
    fn default() -> Self {
        Eyebrow {
            model: Model::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Model
impl Deref for Eyebrow {
    type Target = Model;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl DerefMut for Eyebrow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.model
    }
}

impl BinaryReadable for Eyebrow {
  fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
    Ok(Eyebrow {
      model: Model::read(reader)?,
    })
  }
}