use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyebrowColorWhite {
    pub attribute: Attribute,
}

impl Default for EyebrowColorWhite {
    fn default() -> Self {
        EyebrowColorWhite {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyebrowColorWhite {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyebrowColorWhite {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyebrowColorWhite {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyebrowColorWhite {
            attribute: Attribute::read(reader)?,
        })
    }
}
