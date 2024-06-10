use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyebrowColorG {
    pub attribute: Attribute,
}

impl Default for EyebrowColorG {
    fn default() -> Self {
        EyebrowColorG {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyebrowColorG {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyebrowColorG {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyebrowColorG {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyebrowColorG {
            attribute: Attribute::read(reader)?,
        })
    }
}
