use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct DecalPositionY {
    pub attribute: Attribute,
}

impl Default for DecalPositionY {
    fn default() -> Self {
        DecalPositionY {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for DecalPositionY {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for DecalPositionY {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for DecalPositionY {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(DecalPositionY {
            attribute: Attribute::read(reader)?,
        })
    }
}
