use std::{
    fmt,
    io::{self, Read, Seek},
};

use crate::traits::{binary_readable::BinaryReadable, validate::Validate};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct I64Reader {
    pub data: i64,
}

impl Default for I64Reader {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl fmt::Debug for I64Reader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I64Reader(\n")?;
        for byte in self.data.to_le_bytes() {
            write!(f, "{:02X}\u{2008}", byte)?;
        }
        write!(f, ")")
    }
}

impl BinaryReadable for I64Reader {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mutable_default = &mut [0u8; 8];
        reader
            .read_exact(mutable_default)
            .expect("data should be present");
        Ok(I64Reader {
            data: i64::from_le_bytes(*mutable_default),
        })
    }
}

impl Validate for I64Reader {
    fn validate(&self) -> bool {
        self.data.to_le_bytes().len() == 8
    }
}
