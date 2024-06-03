use std::{
    fmt,
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

use crate::{models::shared::i32_reader::I32Reader, traits::{binary_readable::BinaryReadable, validate::Validate}};

#[derive(serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct BaseMaxFP {
    inner: I32Reader,
}

// Implement Deref trait for BaseMaxFP
impl Deref for BaseMaxFP {
    type Target = I32Reader;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Implement DerefMut trait for BaseMaxFP
impl DerefMut for BaseMaxFP {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// Implement Debug trait for BaseMaxFP by forwarding to I32Reader
impl fmt::Debug for BaseMaxFP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

// Implement BinaryReadable trait for BaseMaxFP by forwarding to I32Reader
impl BinaryReadable for BaseMaxFP {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(BaseMaxFP {
            inner: I32Reader::read(reader)?,
        })
    }
}

impl Validate for BaseMaxFP {
    fn validate(&self) -> bool {
        true
    }
}
