use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct InnerBrowHeight {
    pub attribute: Attribute,
}

impl Default for InnerBrowHeight {
    fn default() -> Self {
        InnerBrowHeight {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for InnerBrowHeight {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for InnerBrowHeight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for InnerBrowHeight {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(InnerBrowHeight {
            attribute: Attribute::read(reader)?,
        })
    }
}
