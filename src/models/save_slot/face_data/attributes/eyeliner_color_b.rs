use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyelinerColorB {
    pub attribute: Attribute,
}

impl Default for EyelinerColorB {
    fn default() -> Self {
        EyelinerColorB {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyelinerColorB {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyelinerColorB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyelinerColorB {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyelinerColorB {
            attribute: Attribute::read(reader)?,
        })
    }
}
