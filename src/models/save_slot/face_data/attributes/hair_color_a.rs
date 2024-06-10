use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct HairColorA {
    pub attribute: Attribute,
}

impl Default for HairColorA {
    fn default() -> Self {
        HairColorA {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for HairColorA {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for HairColorA {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for HairColorA {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(HairColorA {
            attribute: Attribute::read(reader)?,
        })
    }
}
