use crate::traits::byte_array_readable::ByteArrayReadable;
use crate::{models::shared::byte_array_reader::ByteArray, traits::validate::Validate};
use std::io::{self, Read, Seek};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Unk24Bytes {
    pub data: ByteArray<24>,
}

impl Unk24Bytes {
    pub fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let data = ByteArray::<24>::read(reader)?;
        Ok(Unk24Bytes { data })
    }
}

impl Validate for Unk24Bytes {
    fn validate(&self) -> bool {
        self.data.validate()
    }
}
