use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct MouthExpression {
    pub attribute: Attribute,
}

impl Default for MouthExpression {
    fn default() -> Self {
        MouthExpression {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for MouthExpression {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for MouthExpression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for MouthExpression {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(MouthExpression {
            attribute: Attribute::read(reader)?,
        })
    }
}
