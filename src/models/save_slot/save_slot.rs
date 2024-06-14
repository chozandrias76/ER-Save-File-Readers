use std::{
    fmt,
    io::{self, Read, Seek},
};

use crate::{
    models::shared::byte_array_reader::ByteArray,
    traits::{binary_readable::BinaryReadable, validate::Validate},
};

use super::{checksum::Checksum, unk_24_bytes::Unk24Bytes};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SaveSlot {
    pub checksum: Checksum,
    pub unk0x16: Unk24Bytes,
}

impl SaveSlot {
    pub fn length(&self) -> usize {
        // let base_size = mem::size_of::<Checksum>();

        2621456 // - base_size
    }
}

impl Default for SaveSlot {
    fn default() -> Self {
        Self {
            checksum: Default::default(),
            unk0x16: Unk24Bytes {
                data: ByteArray {
                    data: vec![0u8; 0x18],
                },
            },
        }
    }
}

impl fmt::Debug for SaveSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SaveSlot(\n")?;
        write!(f, "checksum: {:?}", self.checksum)?;
        write!(f, "\n)")
    }
}

impl BinaryReadable for SaveSlot {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let checksum_data = &mut Self::default().checksum.data;
        reader
            .read_exact(checksum_data)
            .expect("checksum_data should be present");
        let mut instance = Self::default();
        instance.checksum.data = *checksum_data;

        Ok(instance)
    }
}

impl Validate for SaveSlot {
    fn validate(&self) -> bool {
        self.length() == 2621456
    }
}
