use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyelashColorG {
    pub attribute: Attribute,
}

impl Default for EyelashColorG {
    fn default() -> Self {
        EyelashColorG {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyelashColorG {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyelashColorG {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyelashColorG {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyelashColorG {
            attribute: Attribute::read(reader)?,
        })
    }
}
