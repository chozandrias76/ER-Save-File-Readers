use std::io::{self, Read, Seek};

use crate::traits::validate::Validate;

pub trait ByteArrayReader {
    fn read<R: Read + Seek>(reader: &mut R, length: usize) -> io::Result<Self>
    where
        Self: Sized;
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ByteArray {
    pub data: Vec<u8>,
    pub length: usize,
}

impl ByteArrayReader for ByteArray {
    fn read<R: Read + Seek>(reader: &mut R, length: usize) -> io::Result<Self> {
        let mut data = vec![0u8; length];
        reader.read_exact(&mut data)?;
        Ok(ByteArray { data, length })
    }
}

impl Validate for ByteArray {
    fn validate(&self) -> bool {
        self.data.len() == self.length
    }
}
