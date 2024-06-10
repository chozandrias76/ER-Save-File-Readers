use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct LipstickColorR {
    pub attribute: Attribute,
}

impl Default for LipstickColorR {
    fn default() -> Self {
        LipstickColorR {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for LipstickColorR {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for LipstickColorR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for LipstickColorR {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(LipstickColorR {
            attribute: Attribute::read(reader)?,
        })
    }
}
