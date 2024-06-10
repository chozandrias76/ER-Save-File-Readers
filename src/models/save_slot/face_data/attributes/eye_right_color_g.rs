use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyeRightColorG {
    pub attribute: Attribute,
}

impl Default for EyeRightColorG {
    fn default() -> Self {
        EyeRightColorG {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyeRightColorG {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyeRightColorG {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyeRightColorG {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyeRightColorG {
            attribute: Attribute::read(reader)?,
        })
    }
}
