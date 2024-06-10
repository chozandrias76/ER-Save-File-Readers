use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct NoseBridgeWidth {
    pub attribute: Attribute,
}

impl Default for NoseBridgeWidth {
    fn default() -> Self {
        NoseBridgeWidth {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for NoseBridgeWidth {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for NoseBridgeWidth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for NoseBridgeWidth {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(NoseBridgeWidth {
            attribute: Attribute::read(reader)?,
        })
    }
}
