use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct Eyeliner {
    pub attribute: Attribute,
}

impl Default for Eyeliner {
    fn default() -> Self {
        Eyeliner {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for Eyeliner {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for Eyeliner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for Eyeliner {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(Eyeliner {
            attribute: Attribute::read(reader)?,
        })
    }
}
