use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct FrenziedFlame {
    pub attribute: Attribute,
}

impl Default for FrenziedFlame {
    fn default() -> Self {
        FrenziedFlame {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for FrenziedFlame {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for FrenziedFlame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for FrenziedFlame {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(FrenziedFlame {
            attribute: Attribute::read(reader)?,
        })
    }
}
