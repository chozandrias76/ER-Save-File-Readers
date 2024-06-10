use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct Cheeks {
    pub attribute: Attribute,
}

impl Default for Cheeks {
    fn default() -> Self {
        Cheeks {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for Cheeks {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for Cheeks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for Cheeks {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(Cheeks {
            attribute: Attribute::read(reader)?,
        })
    }
}
