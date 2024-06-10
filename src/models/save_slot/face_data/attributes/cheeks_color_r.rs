use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct CheeksColorR {
    pub attribute: Attribute,
}

impl Default for CheeksColorR {
    fn default() -> Self {
        CheeksColorR {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for CheeksColorR {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for CheeksColorR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for CheeksColorR {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(CheeksColorR {
            attribute: Attribute::read(reader)?,
        })
    }
}