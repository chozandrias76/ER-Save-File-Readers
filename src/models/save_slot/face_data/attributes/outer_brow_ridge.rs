use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct OuterBrowRidge {
    pub attribute: Attribute,
}

impl Default for OuterBrowRidge {
    fn default() -> Self {
        OuterBrowRidge {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for OuterBrowRidge {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for OuterBrowRidge {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for OuterBrowRidge {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(OuterBrowRidge {
            attribute: Attribute::read(reader)?,
        })
    }
}
