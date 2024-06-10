use crate::traits::binary_readable::BinaryReadable;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

use super::attribute::Attribute;

pub struct ApparentAge {
    pub attribute: Attribute,
}

impl Default for ApparentAge {
    fn default() -> Self {
        ApparentAge {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for ApparentAge {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for ApparentAge {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for ApparentAge {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(ApparentAge {
            attribute: Attribute::read(reader)?,
        })
    }
}
