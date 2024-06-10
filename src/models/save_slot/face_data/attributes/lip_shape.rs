use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct LipShape {
    pub attribute: Attribute,
}

impl Default for LipShape {
    fn default() -> Self {
        LipShape {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for LipShape {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for LipShape {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for LipShape {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(LipShape {
            attribute: Attribute::read(reader)?,
        })
    }
}
