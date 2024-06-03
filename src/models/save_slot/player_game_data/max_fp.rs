use std::{
    fmt,
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

use crate::{models::shared::i32_reader::I32Reader, traits::binary_readable::BinaryReadable};

#[derive(serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct MaxFP {
    inner: I32Reader,
}

// Implement Deref trait for MaxFP
impl Deref for MaxFP {
    type Target = I32Reader;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Implement DerefMut trait for MaxFP
impl DerefMut for MaxFP {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// Implement Debug trait for MaxFP by forwarding to I32Reader
impl fmt::Debug for MaxFP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

// Implement BinaryReadable trait for MaxFP by forwarding to I32Reader
impl BinaryReadable for MaxFP {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(MaxFP {
            inner: I32Reader::read(reader)?,
        })
    }
}
