use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct SkinDarkCircleColorG {
    pub attribute: Attribute,
}

impl Default for SkinDarkCircleColorG {
    fn default() -> Self {
        SkinDarkCircleColorG {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for SkinDarkCircleColorG {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for SkinDarkCircleColorG {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for SkinDarkCircleColorG {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(SkinDarkCircleColorG {
            attribute: Attribute::read(reader)?,
        })
    }
}
