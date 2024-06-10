use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct BodyScaleHead {
    pub attribute: Attribute,
}

impl Default for BodyScaleHead {
    fn default() -> Self {
        BodyScaleHead {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for BodyScaleHead {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for BodyScaleHead {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for BodyScaleHead {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(BodyScaleHead {
            attribute: Attribute::read(reader)?,
        })
    }
}
