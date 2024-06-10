use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct SkinColorA {
    pub attribute: Attribute,
}

impl Default for SkinColorA {
    fn default() -> Self {
        SkinColorA {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for SkinColorA {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for SkinColorA {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for SkinColorA {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(SkinColorA {
            attribute: Attribute::read(reader)?,
        })
    }
}
