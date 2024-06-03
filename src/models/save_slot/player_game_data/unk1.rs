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
pub struct Unk1 {
    inner: I32Reader,
}

// Implement Deref trait for Unk1
impl Deref for Unk1 {
    type Target = I32Reader;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Implement DerefMut trait for Unk1
impl DerefMut for Unk1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// Implement Debug trait for Unk1 by forwarding to I32Reader
impl fmt::Debug for Unk1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

// Implement BinaryReadable trait for Unk1 by forwarding to I32Reader
impl BinaryReadable for Unk1 {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(Unk1 {
            inner: I32Reader::read(reader)?,
        })
    }
}

impl Validate for Unk1 {
    fn validate(&self) -> bool {
        self.inner.validate()
    }
}
