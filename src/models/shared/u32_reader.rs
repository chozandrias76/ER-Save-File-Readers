use std::{
    fmt,
    io::{self, Read, Seek},
};

use crate::traits::{binary_readable::BinaryReadable, validate::Validate};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct U32Reader {
    pub data: u32,
}

impl Default for U32Reader {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl fmt::Debug for U32Reader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U32Reader(\n")?;
        for byte in self.data.to_le_bytes() {
            write!(f, "{:02X}\u{2008}", byte)?;
        }
        write!(f, ")")
    }
}

impl BinaryReadable for U32Reader {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mutable_default = &mut [0u8; 4]; // Create a mutable reference to a [u8] slice
        match reader
            .read_exact(mutable_default) {
                Ok(_) => Ok(U32Reader {
                    data: u32::from_le_bytes(*mutable_default),
                }),
                Err(e) => return Err(e),
            }
        
    }
}

impl Validate for U32Reader {
    fn validate(&self) -> bool {
        self.data.to_le_bytes().len() == 4
    }
}
