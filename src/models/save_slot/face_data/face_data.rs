use std::{
    fmt,
    io::{self, Read, Seek},
};

use crate::{
    models::shared::byte_array_reader::{ByteArray, ByteArrayReadable},
    traits::{binary_readable::BinaryReadable, validate::Validate},
};

use super::{
    attributes::apparent_age::ApparentAge, magic_bytes::MagicBytes, models::{
        accessory::Accessory, beard::Beard, decal::Decal, eye::Eye, eyebrow::Eyebrow,
        eyelash::Eyelash, face::Face, hair::Hair,
    }
};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct FaceData {
    pub data: ByteArray<303>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct FaceDatum {
    pub datum: (
        MagicBytes,
        Face,
        Hair,
        Eye,
        Eyebrow,
        Beard,
        Accessory,
        Decal,
        Eyelash,
        ApparentAge,
    ),
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
            data: ByteArray::<303>::read(reader)?,
        })
    }
}

impl Validate for FaceData {
    fn validate(&self) -> bool {
        self.data.validate()
    }
}
