use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct SkinDarkCircle {
    pub attribute: Attribute,
}

impl Default for SkinDarkCircle {
    fn default() -> Self {
        SkinDarkCircle {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for SkinDarkCircle {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for SkinDarkCircle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for SkinDarkCircle {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(SkinDarkCircle {
            attribute: Attribute::read(reader)?,
        })
    }
}
