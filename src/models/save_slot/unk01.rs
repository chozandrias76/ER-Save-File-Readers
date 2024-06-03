use std::{
    fmt,
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

use crate::{
    models::shared::u32_reader::U32Reader,
    traits::{binary_readable::BinaryReadable, validate::Validate},
};

#[derive(serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct Unk01 {
    inner: U32Reader,
}

// Implement Deref trait for Unk01
impl Deref for Unk01 {
    type Target = U32Reader;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Implement DerefMut trait for Unk01
impl DerefMut for Unk01 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// Implement Debug trait for Unk01 by forwarding to U32Reader
impl fmt::Debug for Unk01 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

// Implement BinaryReadable trait for Unk01 by forwarding to U32Reader
impl BinaryReadable for Unk01 {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(Unk01 {
            inner: U32Reader::read(reader)?,
        })
    }
}

impl Validate for Unk01 {
    fn validate(&self) -> bool {
        self.inner.data.to_le_bytes().len() == 4
    }
}
