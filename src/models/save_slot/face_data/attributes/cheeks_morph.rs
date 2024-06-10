use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct CheeksMorph {
    pub attribute: Attribute,
}

impl Default for CheeksMorph {
    fn default() -> Self {
        CheeksMorph {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for CheeksMorph {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for CheeksMorph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for CheeksMorph {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(CheeksMorph {
            attribute: Attribute::read(reader)?,
        })
    }
}
