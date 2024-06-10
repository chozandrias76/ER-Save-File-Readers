use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct DecalPositionX {
    pub attribute: Attribute,
}

impl Default for DecalPositionX {
    fn default() -> Self {
        DecalPositionX {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for DecalPositionX {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for DecalPositionX {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for DecalPositionX {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(DecalPositionX {
            attribute: Attribute::read(reader)?,
        })
    }
}
