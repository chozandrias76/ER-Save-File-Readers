use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyelashColorB {
    pub attribute: Attribute,
}

impl Default for EyelashColorB {
    fn default() -> Self {
        EyelashColorB {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyelashColorB {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyelashColorB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyelashColorB {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyelashColorB {
            attribute: Attribute::read(reader)?,
        })
    }
}
