use std::io::{self, Read, Seek};

use crate::{models::shared::u8_reader::U8Reader, traits::binary_readable::BinaryReadable};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Attribute {
    pub data: (u8,),
}

// Implement BinaryReadable trait for Attribute by forwarding to U8Reader
impl BinaryReadable for Attribute {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(Attribute {
            data: U8Reader::read(reader)?.data,
        })
    }
}

impl Default for Attribute {
    fn default() -> Self {
        Attribute {
            data: U8Reader::default().data,
        }
    }
}
