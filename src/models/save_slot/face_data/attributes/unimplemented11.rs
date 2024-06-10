use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct Unimplemented11 {
    pub attribute: Attribute,
}

impl Default for Unimplemented11 {
    fn default() -> Self {
        Unimplemented11 {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for Unimplemented11 {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for Unimplemented11 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for Unimplemented11 {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(Unimplemented11 {
            attribute: Attribute::read(reader)?,
        })
    }
}
