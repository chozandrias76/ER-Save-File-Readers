use std::{
    fmt,
    io::{self, Read, Seek},
};

use crate::{models::shared::byte_array_reader::{ByteArray, ByteArrayReader}, traits::{binary_readable::BinaryReadable, validate::Validate}};

pub struct FaceData {
    pub data: ByteArray,
}

impl Default for FaceData {
    fn default() -> Self {
        FaceData {
            data: ByteArray::default(),
        }
    }
}

impl fmt::Debug for FaceData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FaceData(\n")?;

        for byte in self.data.data.iter() {
            write!(f, "\t{:02X}\u{2008}", byte)?;
        }

        write!(f, ")")
    }
}

impl BinaryReadable for FaceData {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(FaceData {
            data: ByteArray::read(reader, 303)?,
        })
    }
}

impl Validate for FaceData {
    fn validate(&self) -> bool {
        self.data.validate()
    }
}
