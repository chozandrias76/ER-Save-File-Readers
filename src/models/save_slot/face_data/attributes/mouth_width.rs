use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct MouthWidth {
    pub attribute: Attribute,
}

impl Default for MouthWidth {
    fn default() -> Self {
        MouthWidth {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for MouthWidth {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for MouthWidth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for MouthWidth {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(MouthWidth {
            attribute: Attribute::read(reader)?,
        })
    }
}
