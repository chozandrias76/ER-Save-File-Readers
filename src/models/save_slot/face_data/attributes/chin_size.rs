use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct ChinSize {
    pub attribute: Attribute,
}

impl Default for ChinSize {
    fn default() -> Self {
        ChinSize {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for ChinSize {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for ChinSize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for ChinSize {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(ChinSize {
            attribute: Attribute::read(reader)?,
        })
    }
}
