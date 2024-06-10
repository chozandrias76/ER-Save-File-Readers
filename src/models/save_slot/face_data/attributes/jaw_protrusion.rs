use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct JawProtrusion {
    pub attribute: Attribute,
}

impl Default for JawProtrusion {
    fn default() -> Self {
        JawProtrusion {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for JawProtrusion {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for JawProtrusion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for JawProtrusion {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(JawProtrusion {
            attribute: Attribute::read(reader)?,
        })
    }
}
