use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct Unimplemented10 {
    pub attribute: Attribute,
}

impl Default for Unimplemented10 {
    fn default() -> Self {
        Unimplemented10 {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for Unimplemented10 {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for Unimplemented10 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for Unimplemented10 {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(Unimplemented10 {
            attribute: Attribute::read(reader)?,
        })
    }
}
