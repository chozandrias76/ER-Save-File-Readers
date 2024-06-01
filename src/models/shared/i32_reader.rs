use std::{
    fmt,
    io::{self, Read, Seek},
};

use crate::traits::binary_readable::BinaryReadable;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct I32Reader {
    pub data: i32,
}

impl Default for I32Reader {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl fmt::Debug for I32Reader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I32Reader(\n")?;
        for byte in self.data.to_le_bytes() {
            write!(f, "{:02X}\u{2008}", byte)?;
        }
        write!(f, ")")
    }
}

impl BinaryReadable for I32Reader {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mutable_default = &mut [0u8; 4]; // Change the type from [i8; 4] to [u8; 4]
        reader
            .read_exact(mutable_default)
            .expect("data should be present");
        Ok(I32Reader {
            data: i32::from_le_bytes(*mutable_default), // Convert the [u8] slice to i32
        })
    }
}
