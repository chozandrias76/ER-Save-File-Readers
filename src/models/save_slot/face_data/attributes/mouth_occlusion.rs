use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct MouthOcclusion {
    pub attribute: Attribute,
}

impl Default for MouthOcclusion {
    fn default() -> Self {
        MouthOcclusion {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for MouthOcclusion {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for MouthOcclusion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for MouthOcclusion {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(MouthOcclusion {
            attribute: Attribute::read(reader)?,
        })
    }
}
