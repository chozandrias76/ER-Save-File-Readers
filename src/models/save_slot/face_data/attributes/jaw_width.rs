use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct JawWidth {
    pub attribute: Attribute,
}

impl Default for JawWidth {
    fn default() -> Self {
        JawWidth {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for JawWidth {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for JawWidth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for JawWidth {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(JawWidth {
            attribute: Attribute::read(reader)?,
        })
    }
}
