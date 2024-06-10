use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct EyeLeftCloudingB {
    pub attribute: Attribute,
}

impl Default for EyeLeftCloudingB {
    fn default() -> Self {
        EyeLeftCloudingB {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for EyeLeftCloudingB {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for EyeLeftCloudingB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for EyeLeftCloudingB {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(EyeLeftCloudingB {
            attribute: Attribute::read(reader)?,
        })
    }
}
