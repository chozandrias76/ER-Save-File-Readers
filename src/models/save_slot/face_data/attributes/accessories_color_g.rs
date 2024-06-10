use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct AccessoriesColorG {
    pub attribute: Attribute,
}

impl Default for AccessoriesColorG {
    fn default() -> Self {
        AccessoriesColorG {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for AccessoriesColorG {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for AccessoriesColorG {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for AccessoriesColorG {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(AccessoriesColorG {
            attribute: Attribute::read(reader)?,
        })
    }
}
