use crate::{
    models::shared::byte_array_reader::{ByteArray, ByteArrayReadable},
    traits::binary_readable::BinaryReadable,
};

use std::{
    io::{self, Read, Seek},
    ops::{Deref, DerefMut},
};

/**
 Unimplemented4 represents a collection of FaceGen attributes that are not
 writable by the user.
 */
pub struct Unimplemented4 {
    pub attributes: ByteArray<64>,
}

impl Default for Unimplemented4 {
    fn default() -> Self {
        Unimplemented4 {
            attributes: ByteArray {
                data: [0; 64].to_vec(),
            },
        }
    }
}

// Implement Deref and DerefMut to delegate field access to ByteArray
impl Deref for Unimplemented4 {
    type Target = ByteArray<64>;

    fn deref(&self) -> &Self::Target {
        &self.attributes
    }
}

impl DerefMut for Unimplemented4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.attributes
    }
}

impl BinaryReadable for Unimplemented4 {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        match ByteArray::<64>::read(reader) {
            Ok(data) => Ok(Unimplemented4 {
                attributes: data,
            }),
            Err(e) => Err(e),
        }
    }
}
