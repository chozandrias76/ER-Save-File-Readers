use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyeRightScleraR {
    pub attribute: Attribute,
}

impl Default for EyeRightScleraR {
    fn default() -> Self {
        EyeRightScleraR {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyeRightScleraR {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyeRightScleraR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyeRightScleraR {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyeRightScleraR {
            attribute: Attribute::read(reader)?,
        })
    }
}
