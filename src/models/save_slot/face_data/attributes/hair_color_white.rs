use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct HairColorWhite {
    pub attribute: Attribute,
}

impl Default for HairColorWhite {
    fn default() -> Self {
        HairColorWhite {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for HairColorWhite {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for HairColorWhite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for HairColorWhite {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(HairColorWhite {
            attribute: Attribute::read(reader)?,
        })
    }
}
