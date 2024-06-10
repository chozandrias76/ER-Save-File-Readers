use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct BeardColorA {
    pub attribute: Attribute,
}

impl Default for BeardColorA {
    fn default() -> Self {
        BeardColorA {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for BeardColorA {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for BeardColorA {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for BeardColorA {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(BeardColorA {
            attribute: Attribute::read(reader)?,
        })
    }
}
