use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyeLeftPosition {
    pub attribute: Attribute,
}

impl Default for EyeLeftPosition {
    fn default() -> Self {
        EyeLeftPosition {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyeLeftPosition {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyeLeftPosition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyeLeftPosition {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyeLeftPosition {
            attribute: Attribute::read(reader)?,
        })
    }
}
