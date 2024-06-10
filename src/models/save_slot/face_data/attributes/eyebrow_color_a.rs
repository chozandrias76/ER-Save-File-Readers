use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyebrowColorA {
    pub attribute: Attribute,
}

impl Default for EyebrowColorA {
    fn default() -> Self {
        EyebrowColorA {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyebrowColorA {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyebrowColorA {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyebrowColorA {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyebrowColorA {
            attribute: Attribute::read(reader)?,
        })
    }
}
