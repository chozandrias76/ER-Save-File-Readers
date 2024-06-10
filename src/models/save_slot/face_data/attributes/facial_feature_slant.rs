use crate::traits::binary_readable::BinaryReadable;

use super::attribute::Attribute;
use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

pub struct FacialFeatureSlant {
    pub attribute: Attribute,
}

impl Default for FacialFeatureSlant {
    fn default() -> Self {
        FacialFeatureSlant {
            attribute: Attribute::default(),
        }
    }
}

// Implement Deref and DerefMut to delegate field access to Attribute
impl Deref for FacialFeatureSlant {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.attribute
    }
}

impl DerefMut for FacialFeatureSlant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attribute
    }
}

impl BinaryReadable for FacialFeatureSlant {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(FacialFeatureSlant {
            attribute: Attribute::read(reader)?,
        })
    }
}
