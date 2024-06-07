use std::{fmt, io::{self, Read, Seek}};

use crate::{
    models::shared::byte_array_reader::{ByteArray, ByteArrayReader},
    traits::validate::Validate,
};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct Unk288Bytes {
    pub data: ByteArray,
}

impl Default for Unk288Bytes {
    fn default() -> Self {
        Unk288Bytes {
            data: ByteArray {
                data: vec![0u8; 288],
                length: 288,
            },
        }
    }
}

impl Unk288Bytes {
    pub fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let byte_length = 288;
        let data = ByteArray::read(reader, byte_length)?;
        Ok(Unk288Bytes { data })
    }
}

impl Validate for Unk288Bytes {
    fn validate(&self) -> bool {
        self.data.validate()
    }
}

impl std::fmt::Debug for Unk288Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unk288Bytes(\n")?;
        fmt::Debug::fmt(&self.data, f)?;
        write!(f, ")")
    }
}
