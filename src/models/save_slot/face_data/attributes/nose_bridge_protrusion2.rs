use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct NoseBridgeProtrusion2 {
    pub attribute: Attribute,
}

impl Default for NoseBridgeProtrusion2 {
    fn default() -> Self {
        NoseBridgeProtrusion2 {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for NoseBridgeProtrusion2 {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for NoseBridgeProtrusion2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for NoseBridgeProtrusion2 {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(NoseBridgeProtrusion2 {
            attribute: Attribute::read(reader)?,
        })
    }
}
