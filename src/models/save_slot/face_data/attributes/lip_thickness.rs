use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct LipThickness {
    pub attribute: Attribute,
}

impl Default for LipThickness {
    fn default() -> Self {
        LipThickness {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for LipThickness {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for LipThickness {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for LipThickness {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(LipThickness {
            attribute: Attribute::read(reader)?,
        })
    }
}
