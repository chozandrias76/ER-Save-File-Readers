use std::{
    fmt,
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

use crate::{models::shared::u32_reader::U32Reader, traits::binary_readable::BinaryReadable};

#[derive(serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct MapID {
    inner: U32Reader,
}

// Implement Deref trait for MapID
impl Deref for MapID {
    type Target = U32Reader;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Implement DerefMut trait for MapID
impl DerefMut for MapID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// Implement Debug trait for MapID by forwarding to U32Reader
impl fmt::Debug for MapID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.inner, f)
    }
}

// Implement BinaryReadable trait for MapID by forwarding to U32Reader
impl BinaryReadable for MapID {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(MapID {
            inner: U32Reader::read(reader)?,
        })
    }
}
