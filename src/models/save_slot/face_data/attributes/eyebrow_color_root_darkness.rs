use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyebrowColorRootDarkness {
    pub attribute: Attribute,
}

impl Default for EyebrowColorRootDarkness {
    fn default() -> Self {
        EyebrowColorRootDarkness {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyebrowColorRootDarkness {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyebrowColorRootDarkness {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyebrowColorRootDarkness {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyebrowColorRootDarkness {
            attribute: Attribute::read(reader)?,
        })
    }
}
