use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct HairColorRootDarkness {
    pub attribute: Attribute,
}

impl Default for HairColorRootDarkness {
    fn default() -> Self {
        HairColorRootDarkness {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for HairColorRootDarkness {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for HairColorRootDarkness {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for HairColorRootDarkness {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(HairColorRootDarkness {
            attribute: Attribute::read(reader)?,
        })
    }
}
