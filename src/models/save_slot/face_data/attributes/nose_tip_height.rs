use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct NoseTipHeight {
    pub attribute: Attribute,
}

impl Default for NoseTipHeight {
    fn default() -> Self {
        NoseTipHeight {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for NoseTipHeight {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for NoseTipHeight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for NoseTipHeight {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(NoseTipHeight {
            attribute: Attribute::read(reader)?,
        })
    }
}
