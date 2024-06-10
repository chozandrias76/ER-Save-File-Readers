use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct BeardStubble {
    pub attribute: Attribute,
}

impl Default for BeardStubble {
    fn default() -> Self {
        BeardStubble {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for BeardStubble {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for BeardStubble {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for BeardStubble {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(BeardStubble {
            attribute: Attribute::read(reader)?,
        })
    }
}
