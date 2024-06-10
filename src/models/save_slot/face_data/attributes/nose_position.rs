use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct NosePosition {
    pub attribute: Attribute,
}

impl Default for NosePosition {
    fn default() -> Self {
        NosePosition {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for NosePosition {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for NosePosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for NosePosition {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(NosePosition {
            attribute: Attribute::read(reader)?,
        })
    }
}
