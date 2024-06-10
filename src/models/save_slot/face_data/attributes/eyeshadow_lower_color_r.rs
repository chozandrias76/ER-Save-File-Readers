use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyeshadowLowerColorR {
    pub attribute: Attribute,
}

impl Default for EyeshadowLowerColorR {
    fn default() -> Self {
        EyeshadowLowerColorR {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyeshadowLowerColorR {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyeshadowLowerColorR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyeshadowLowerColorR {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyeshadowLowerColorR {
            attribute: Attribute::read(reader)?,
        })
    }
}
