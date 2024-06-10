use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct NoseBridgeProtrusion1 {
    pub attribute: Attribute,
}

impl Default for NoseBridgeProtrusion1 {
    fn default() -> Self {
        NoseBridgeProtrusion1 {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for NoseBridgeProtrusion1 {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for NoseBridgeProtrusion1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for NoseBridgeProtrusion1 {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(NoseBridgeProtrusion1 {
            attribute: Attribute::read(reader)?,
        })
    }
}
