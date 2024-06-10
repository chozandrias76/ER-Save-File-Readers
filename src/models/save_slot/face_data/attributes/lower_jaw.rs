use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct LowerJaw {
    pub attribute: Attribute,
}

impl Default for LowerJaw {
    fn default() -> Self {
        LowerJaw {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for LowerJaw {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for LowerJaw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for LowerJaw {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(LowerJaw {
            attribute: Attribute::read(reader)?,
        })
    }
}
