use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct MouthToChinDistance {
    pub attribute: Attribute,
}

impl Default for MouthToChinDistance {
    fn default() -> Self {
        MouthToChinDistance {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for MouthToChinDistance {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for MouthToChinDistance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for MouthToChinDistance {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(MouthToChinDistance {
            attribute: Attribute::read(reader)?,
        })
    }
}
