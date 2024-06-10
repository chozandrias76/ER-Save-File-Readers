use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct VertFaceRatio {
    pub attribute: Attribute,
}

impl Default for VertFaceRatio {
    fn default() -> Self {
        VertFaceRatio {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for VertFaceRatio {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for VertFaceRatio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for VertFaceRatio {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(VertFaceRatio {
            attribute: Attribute::read(reader)?,
        })
    }
}
