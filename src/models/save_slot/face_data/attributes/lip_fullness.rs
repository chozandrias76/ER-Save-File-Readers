use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct LipFullness {
    pub attribute: Attribute,
}

impl Default for LipFullness {
    fn default() -> Self {
        LipFullness {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for LipFullness {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for LipFullness {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for LipFullness {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(LipFullness {
            attribute: Attribute::read(reader)?,
        })
    }
}