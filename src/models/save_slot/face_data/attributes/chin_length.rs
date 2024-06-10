use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct ChinLength {
    pub attribute: Attribute,
}

impl Default for ChinLength {
    fn default() -> Self {
        ChinLength {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for ChinLength {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for ChinLength {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for ChinLength {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(ChinLength {
            attribute: Attribute::read(reader)?,
        })
    }
}
