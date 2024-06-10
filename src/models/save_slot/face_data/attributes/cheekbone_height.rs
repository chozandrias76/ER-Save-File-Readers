use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct CheekboneHeight {
    pub attribute: Attribute,
}

impl Default for CheekboneHeight {
    fn default() -> Self {
        CheekboneHeight {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for CheekboneHeight {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for CheekboneHeight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for CheekboneHeight {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(CheekboneHeight {
            attribute: Attribute::read(reader)?,
        })
    }
}
