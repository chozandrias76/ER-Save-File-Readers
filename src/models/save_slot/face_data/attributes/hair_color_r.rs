use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct HairColorR {
    pub attribute: Attribute,
}

impl Default for HairColorR {
    fn default() -> Self {
        HairColorR {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for HairColorR {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for HairColorR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for HairColorR {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(HairColorR {
            attribute: Attribute::read(reader)?,
        })
    }
}
