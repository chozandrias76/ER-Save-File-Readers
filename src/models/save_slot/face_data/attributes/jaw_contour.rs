use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct JawContour {
    pub attribute: Attribute,
}

impl Default for JawContour {
    fn default() -> Self {
        JawContour {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for JawContour {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for JawContour {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for JawContour {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(JawContour {
            attribute: Attribute::read(reader)?,
        })
    }
}
