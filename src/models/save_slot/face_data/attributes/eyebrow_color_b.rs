use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyebrowColorB {
    pub attribute: Attribute,
}

impl Default for EyebrowColorB {
    fn default() -> Self {
        EyebrowColorB {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyebrowColorB {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyebrowColorB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyebrowColorB {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyebrowColorB {
            attribute: Attribute::read(reader)?,
        })
    }
}
