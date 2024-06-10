use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct LipSize {
    pub attribute: Attribute,
}

impl Default for LipSize {
    fn default() -> Self {
        LipSize {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for LipSize {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for LipSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for LipSize {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(LipSize {
            attribute: Attribute::read(reader)?,
        })
    }
}
