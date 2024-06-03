use std::{
    fmt,
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

use crate::{models::shared::i32_reader::I32Reader, traits::{binary_readable::BinaryReadable, validate::Validate}};

#[derive(serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct MaxHealth {
    inner: I32Reader,
}

// Implement Deref trait for MaxHealth
impl Deref for MaxHealth {
    type Target = I32Reader;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Implement DerefMut trait for MaxHealth
impl DerefMut for MaxHealth {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// Implement Debug trait for MaxHealth by forwarding to I32Reader
impl fmt::Debug for MaxHealth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

// Implement BinaryReadable trait for MaxHealth by forwarding to I32Reader
impl BinaryReadable for MaxHealth {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(MaxHealth {
            inner: I32Reader::read(reader)?,
        })
    }
}

impl Validate for MaxHealth {
    fn validate(&self) -> bool {
        true
    }
}
