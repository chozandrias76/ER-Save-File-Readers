use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct Numen {
    pub attribute: Attribute,
}

impl Default for Numen {
    fn default() -> Self {
        Numen {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for Numen {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for Numen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for Numen {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(Numen {
            attribute: Attribute::read(reader)?,
        })
    }
}
