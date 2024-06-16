use std::io::{self, Read, Seek};

use crate::traits::{binary_readable::BinaryReadable, validate::Validate};

impl BinaryReadable for i16 {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mutable_default = &mut [0u8; 2];
        reader
            .read_exact(mutable_default)
            .expect("data should be present");
        Ok(i16::from_le_bytes(*mutable_default))
    }
}

impl Validate for i16 {
    fn validate(&self) -> bool {
        self.to_le_bytes().len() == 2
    }
}
