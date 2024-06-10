use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct Lipstick {
    pub attribute: Attribute,
}

impl Default for Lipstick {
    fn default() -> Self {
        Lipstick {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for Lipstick {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for Lipstick {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for Lipstick {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(Lipstick {
            attribute: Attribute::read(reader)?,
        })
    }
}
