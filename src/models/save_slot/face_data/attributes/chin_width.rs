use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct ChinWidth {
    pub attribute: Attribute,
}

impl Default for ChinWidth {
    fn default() -> Self {
        ChinWidth {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for ChinWidth {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for ChinWidth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for ChinWidth {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(ChinWidth {
            attribute: Attribute::read(reader)?,
        })
    }
}
