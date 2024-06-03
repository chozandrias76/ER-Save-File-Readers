use std::{
    fmt,
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

use crate::{
    models::shared::i32_reader::I32Reader,
    traits::{binary_readable::BinaryReadable, validate::Validate},
};

#[derive(serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct MaxBaseHealth {
    inner: I32Reader,
}

// Implement Deref trait for MaxBaseHealth
impl Deref for MaxBaseHealth {
    type Target = I32Reader;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Implement DerefMut trait for MaxBaseHealth
impl DerefMut for MaxBaseHealth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// Implement Debug trait for MaxBaseHealth by forwarding to I32Reader
impl fmt::Debug for MaxBaseHealth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

// Implement BinaryReadable trait for MaxBaseHealth by forwarding to I32Reader
impl BinaryReadable for MaxBaseHealth {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(MaxBaseHealth {
            inner: I32Reader::read(reader)?,
        })
    }
}

impl Validate for MaxBaseHealth {
    fn validate(&self) -> bool {
        self.inner.validate()
    }
}
