use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct BeardColorB {
    pub attribute: Attribute,
}

impl Default for BeardColorB {
    fn default() -> Self {
        BeardColorB {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for BeardColorB {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for BeardColorB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for BeardColorB {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(BeardColorB {
            attribute: Attribute::read(reader)?,
        })
    }
}
