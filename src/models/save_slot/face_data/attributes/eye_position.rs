use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyePosition {
    pub attribute: Attribute,
}

impl Default for EyePosition {
    fn default() -> Self {
        EyePosition {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyePosition {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyePosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyePosition {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyePosition {
            attribute: Attribute::read(reader)?,
        })
    }
}
