use crate::models::shared::byte_array_reader::{ByteArray, ByteArrayReadable};
use crate::traits::binary_readable::BinaryReadable;

use std::{
    fmt,
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};


#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct MagicBytes {
    pub data: ByteArray<16>,
}

impl Default for MagicBytes {
    fn default() -> Self {
        MagicBytes {
            data: ByteArray {
                data: [0; 16].to_vec(),
            },
        }
    }
}

// Implement Deref trait for MagicBytes
impl Deref for MagicBytes {
    type Target = ByteArray<16>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// Implement DerefMut trait for MagicBytes
impl DerefMut for MagicBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

// Implement Debug trait for MagicBytes by forwarding to BoolReader
impl fmt::Debug for MagicBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.data, f)
    }
}

// Implement BinaryReadable trait for MagicBytes by forwarding to BoolReader
impl BinaryReadable for MagicBytes {
    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, io::Error> {
        match ByteArray::<16>::read(reader) {
            Ok(data) => Ok(Self {
                data,
            }),
            Err(e) => Err(e),
        }
    }
}
