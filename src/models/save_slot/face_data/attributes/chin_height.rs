use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct ChinHeight {
    pub attribute: Attribute,
}

impl Default for ChinHeight {
    fn default() -> Self {
        ChinHeight {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for ChinHeight {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for ChinHeight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for ChinHeight {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(ChinHeight {
            attribute: Attribute::read(reader)?,
        })
    }
}
