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
pub struct Health {
    inner: I32Reader,
}

// Implement Deref trait for Health
impl Deref for Health {
    type Target = I32Reader;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Implement DerefMut trait for Health
impl DerefMut for Health {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// Implement Debug trait for Health by forwarding to I32Reader
impl fmt::Debug for Health {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

// Implement BinaryReadable trait for Health by forwarding to I32Reader
impl BinaryReadable for Health {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(Health {
            inner: I32Reader::read(reader)?,
        })
    }
}

impl Validate for Health {
    fn validate(&self) -> bool {
        self.inner.validate()
    }
}
