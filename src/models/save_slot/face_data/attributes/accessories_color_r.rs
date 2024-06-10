use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct AccessoriesColorR {
    pub attribute: Attribute,
}

impl Default for AccessoriesColorR {
    fn default() -> Self {
        AccessoriesColorR {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for AccessoriesColorR {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for AccessoriesColorR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for AccessoriesColorR {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(AccessoriesColorR {
            attribute: Attribute::read(reader)?,
        })
    }
}
