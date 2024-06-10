use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct ChinProtrusion {
    pub attribute: Attribute,
}

impl Default for ChinProtrusion {
    fn default() -> Self {
        ChinProtrusion {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for ChinProtrusion {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for ChinProtrusion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for ChinProtrusion {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(ChinProtrusion {
            attribute: Attribute::read(reader)?,
        })
    }
}
