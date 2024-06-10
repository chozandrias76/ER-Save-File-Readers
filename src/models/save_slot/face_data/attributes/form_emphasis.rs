use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct FormEmphasis {
    pub attribute: Attribute,
}

impl Default for FormEmphasis {
    fn default() -> Self {
        FormEmphasis {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for FormEmphasis {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for FormEmphasis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for FormEmphasis {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(FormEmphasis {
            attribute: Attribute::read(reader)?,
        })
    }
}
