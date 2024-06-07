use std::{
    fmt,
    io::{self, Read, Seek},
};

use crate::traits::{binary_readable::BinaryReadable, validate::Validate};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct I16Reader {
    pub data: i16,
}

impl Default for I16Reader {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl fmt::Debug for I16Reader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I16Reader(\n")?;
        for byte in self.data.to_le_bytes() {
            write!(f, "{:02X}\u{2008}", byte)?;
        }
        write!(f, ")")
    }
}

impl BinaryReadable for I16Reader {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mutable_default = &mut [0u8; 2];
        reader
            .read_exact(mutable_default)
            .expect("data should be present");
        Ok(I16Reader {
            data: i16::from_le_bytes(*mutable_default),
        })
    }
}

impl Validate for I16Reader {
    fn validate(&self) -> bool {
        self.data.to_le_bytes().len() == 2
    }
}
