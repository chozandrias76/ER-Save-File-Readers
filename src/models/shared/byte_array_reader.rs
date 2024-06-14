use std::{fmt, io::{self, Read, Seek}};

use crate::traits::validate::Validate;
use crate::traits::byte_array_readable::ByteArrayReadable;

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct ByteArray<const N: usize> {
    pub data: Vec<u8>,
}

impl<const N: usize> Default for ByteArray<N> {
    fn default() -> Self {
        ByteArray {
            data: vec![0x0; N]
        }
    }
}

impl<const N: usize> ByteArrayReadable for ByteArray<N> where [(); N]: {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mut data = vec![0u8; N];
        reader.read_exact(&mut data)?;
        Ok(ByteArray { data })
    }
}

impl<const N: usize> Validate for ByteArray<N> {
    fn validate(&self) -> bool {
        self.data.len() == N
    }
}

impl<const N: usize> fmt::Debug for ByteArray<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ByteArray(\n")?;
        for byte in &self.data {
            write!(f, "{:02X}\u{2008}", byte)?;
        }
        write!(f, ")")
    }
}
