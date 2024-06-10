use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct CheekboneDepth {
    pub attribute: Attribute,
}

impl Default for CheekboneDepth {
    fn default() -> Self {
        CheekboneDepth {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for CheekboneDepth {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for CheekboneDepth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for CheekboneDepth {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(CheekboneDepth {
            attribute: Attribute::read(reader)?,
        })
    }
}
