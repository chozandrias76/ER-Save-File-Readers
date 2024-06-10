use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct MouthProtrusion {
    pub attribute: Attribute,
}

impl Default for MouthProtrusion {
    fn default() -> Self {
        MouthProtrusion {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for MouthProtrusion {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for MouthProtrusion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for MouthProtrusion {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(MouthProtrusion {
            attribute: Attribute::read(reader)?,
        })
    }
}
