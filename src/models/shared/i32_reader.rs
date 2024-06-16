use std::io::{self, Read, Seek};

use crate::traits::{binary_readable::BinaryReadable, validate::Validate};

impl BinaryReadable for i32 {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mutable_default = &mut Self::default().to_le_bytes();
        reader
            .read_exact(mutable_default)
            .expect("data should be present");
        Ok(i32::from_le_bytes(*mutable_default))
    }
}

impl Validate for i32 {
    fn validate(&self) -> bool {
        self.to_le_bytes().len() == 4
    }
}
