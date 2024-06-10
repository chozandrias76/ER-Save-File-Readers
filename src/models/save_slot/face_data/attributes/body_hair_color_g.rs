use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct BodyHairColorG {
    pub attribute: Attribute,
}

impl Default for BodyHairColorG {
    fn default() -> Self {
        BodyHairColorG {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for BodyHairColorG {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for BodyHairColorG {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for BodyHairColorG {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(BodyHairColorG {
            attribute: Attribute::read(reader)?,
        })
    }
}
