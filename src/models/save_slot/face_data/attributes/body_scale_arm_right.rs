use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct BodyScaleArmRight {
    pub attribute: Attribute,
}

impl Default for BodyScaleArmRight {
    fn default() -> Self {
        BodyScaleArmRight {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for BodyScaleArmRight {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for BodyScaleArmRight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for BodyScaleArmRight {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(BodyScaleArmRight {
            attribute: Attribute::read(reader)?,
        })
    }
}
