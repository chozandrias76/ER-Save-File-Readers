use std::{
    fmt,
    io::{self, Read, Seek},
};

use crate::traits::binary_readable::BinaryReadable;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Checksum {
    pub data: [u8; 0x10],
}

impl Default for Checksum {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl fmt::Debug for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Checksum(\n")?;
        for byte in &self.data {
            write!(f, "{:02X}\u{2008}", byte)?;
        }
        write!(f, ")")
    }
}

impl BinaryReadable for Checksum {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mutable_default = &mut Self::default().data;
        reader
            .read_exact(mutable_default)
            .expect("data should be present");
        Ok(Checksum {
            data: *mutable_default,
        })
    }
}
