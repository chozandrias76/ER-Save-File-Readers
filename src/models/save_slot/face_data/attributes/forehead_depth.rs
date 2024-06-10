use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct ForeheadDepth {
    pub attribute: Attribute,
}

impl Default for ForeheadDepth {
    fn default() -> Self {
        ForeheadDepth {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for ForeheadDepth {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for ForeheadDepth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for ForeheadDepth {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(ForeheadDepth {
            attribute: Attribute::read(reader)?,
        })
    }
}
