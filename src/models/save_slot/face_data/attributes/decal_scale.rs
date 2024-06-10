use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct DecalScale {
    pub attribute: Attribute,
}

impl Default for DecalScale {
    fn default() -> Self {
        DecalScale {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for DecalScale {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for DecalScale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for DecalScale {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(DecalScale {
            attribute: Attribute::read(reader)?,
        })
    }
}
