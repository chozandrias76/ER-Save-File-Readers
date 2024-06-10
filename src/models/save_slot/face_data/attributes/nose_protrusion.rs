use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct NoseProtrusion {
    pub attribute: Attribute,
}

impl Default for NoseProtrusion {
    fn default() -> Self {
        NoseProtrusion {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for NoseProtrusion {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for NoseProtrusion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for NoseProtrusion {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(NoseProtrusion {
            attribute: Attribute::read(reader)?,
        })
    }
}
