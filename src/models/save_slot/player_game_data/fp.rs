use std::{
    fmt,
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

use crate::{models::shared::i32_reader::I32Reader, traits::binary_readable::BinaryReadable};

#[derive(serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct FP {
    inner: I32Reader,
}

// Implement Deref trait for FP
impl Deref for FP {
    type Target = I32Reader;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Implement DerefMut trait for FP
impl DerefMut for FP {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// Implement Debug trait for FP by forwarding to I32Reader
impl fmt::Debug for FP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

// Implement BinaryReadable trait for FP by forwarding to I32Reader
impl BinaryReadable for FP {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(FP {
            inner: I32Reader::read(reader)?,
        })
    }
}
