use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct SkinPores {
    pub attribute: Attribute,
}

impl Default for SkinPores {
    fn default() -> Self {
        SkinPores {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for SkinPores {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for SkinPores {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for SkinPores {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(SkinPores {
            attribute: Attribute::read(reader)?,
        })
    }
}
