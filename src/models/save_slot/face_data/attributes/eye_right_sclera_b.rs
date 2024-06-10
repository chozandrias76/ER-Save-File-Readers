use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyeRightScleraB {
    pub attribute: Attribute,
}

impl Default for EyeRightScleraB {
    fn default() -> Self {
        EyeRightScleraB {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyeRightScleraB {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyeRightScleraB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyeRightScleraB {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyeRightScleraB {
            attribute: Attribute::read(reader)?,
        })
    }
}
