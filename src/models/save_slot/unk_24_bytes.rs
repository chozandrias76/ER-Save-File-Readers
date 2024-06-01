use std::io::{self, Read, Seek};

use crate::models::shared::byte_array_reader::{ByteArray, ByteArrayReader};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Unk24Bytes {
    pub data: ByteArray,
}

impl Unk24Bytes {
    pub fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let byte_length = 24;
        let data = ByteArray::read(reader, byte_length)?;
        Ok(Unk24Bytes { data })
    }
}
