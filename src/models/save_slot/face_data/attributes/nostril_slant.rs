use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct NostrilSlant {
    pub attribute: Attribute,
}

impl Default for NostrilSlant {
    fn default() -> Self {
        NostrilSlant {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for NostrilSlant {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for NostrilSlant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for NostrilSlant {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(NostrilSlant {
            attribute: Attribute::read(reader)?,
        })
    }
}
