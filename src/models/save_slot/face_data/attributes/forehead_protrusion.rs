use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct ForeheadProtrusion {
    pub attribute: Attribute,
}

impl Default for ForeheadProtrusion {
    fn default() -> Self {
        ForeheadProtrusion {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for ForeheadProtrusion {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for ForeheadProtrusion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for ForeheadProtrusion {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(ForeheadProtrusion {
            attribute: Attribute::read(reader)?,
        })
    }
}
