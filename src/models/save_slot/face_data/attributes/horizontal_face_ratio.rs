use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct HorizontalFaceRatio {
    pub attribute: Attribute,
}

impl Default for HorizontalFaceRatio {
    fn default() -> Self {
        HorizontalFaceRatio {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for HorizontalFaceRatio {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for HorizontalFaceRatio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for HorizontalFaceRatio {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(HorizontalFaceRatio {
            attribute: Attribute::read(reader)?,
        })
    }
}
