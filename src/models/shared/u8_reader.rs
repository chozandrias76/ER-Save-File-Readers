use std::io::{self, Read, Seek};

use crate::traits::binary_readable::BinaryReadable;

impl BinaryReadable for u8 {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mutable_default: &mut [u8; 1] = &mut [Self::default()];
        match reader.read_exact(mutable_default) {
            Ok(_) => Ok(mutable_default[0]),
            Err(e) => {
                return Err(e);
            }
        }
    }
}
