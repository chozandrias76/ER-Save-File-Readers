use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct BodyScaleLegRight {
    pub attribute: Attribute,
}

impl Default for BodyScaleLegRight {
    fn default() -> Self {
        BodyScaleLegRight {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for BodyScaleLegRight {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for BodyScaleLegRight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for BodyScaleLegRight {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(BodyScaleLegRight {
            attribute: Attribute::read(reader)?,
        })
    }
}
