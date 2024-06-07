use std::{
    fmt,
    io::{self, Read, Seek},
    iter::{self},
    slice,
};

use crate::traits::{binary_readable::BinaryReadable, validate::Validate};

use super::{
    unk_288_bytes::Unk288Bytes, unk_bool::UnkBool, unk_i16::UnkI16, unk_i32::UnkI32,
    unk_i64::UnkI64,
};

pub struct FaceData {
    pub data: (UnkI32, Unk288Bytes, UnkBool, UnkI16, UnkI64),
}

impl Default for FaceData {
    fn default() -> Self {
        FaceData {
            data: (
                UnkI32::default(),
                Unk288Bytes::default(),
                UnkBool::default(),
                UnkI16::default(),
                UnkI64::default(),
            ),
        }
    }
}

impl fmt::Debug for FaceData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FaceData(\n")?;

        write!(f, "\tUnkI32(\n")?;
        for byte in self.data.0.data.to_le_bytes().iter() {
            write!(f, "\t{:02X}\u{2008}", byte)?;
        }
        write!(f, "\t)\n")?;

        write!(f, "{:?}", self.data.1)?;

        write!(f, "\tUnkBool(\n")?;
        for byte in u8::from(self.data.2.data).to_le_bytes().iter() {
            write!(f, "\t{:02X}\u{2008}", byte)?;
        }
        write!(f, "\t)\n")?;

        write!(f, "\tUnkI16(\n")?;
        for byte in i16::from(self.data.3.data).to_le_bytes().iter() {
            write!(f, "\t{:02X}\u{2008}", byte)?;
        }
        write!(f, "\t)\n")?;

        write!(f, "\tUUnkI64(\n")?;
        for byte in i64::from(self.data.4.data).to_le_bytes().iter() {
            write!(f, "\t{:02X}\u{2008}", byte)?;
        }
        write!(f, "\t)\n")?;

        write!(f, ")")
    }
}

impl BinaryReadable for FaceData {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(FaceData {
            data: (
                UnkI32::read(reader)?,
                Unk288Bytes::read(reader)?,
                UnkBool::read(reader)?,
                UnkI16::read(reader)?,
                UnkI64::read(reader)?,
            ),
        })
    }
}

impl Validate for FaceData {
    fn validate(&self) -> bool {
        self.data.0.validate()
            && self.data.1.validate()
            && self.data.2.validate()
            && self.data.3.validate()
            && self.data.4.validate()
    }
}

impl IntoIterator for FaceData {
    type Item = (usize, u8);
    type IntoIter = iter::Enumerate<
        std::iter::FlatMap<
            std::vec::IntoIter<&'static [u8]>,
            std::iter::Copied<slice::Iter<'static, u8>>,
            fn(&'static [u8]) -> std::iter::Copied<slice::Iter<'static, u8>>,
        >,
    >;

    fn into_iter(self) -> Self::IntoIter {
        let iterators: Vec<&'static [u8]> = vec![
            Box::leak(
                self.data
                    .0
                    .data
                    .to_le_bytes()
                    .as_slice()
                    .to_owned()
                    .into_boxed_slice(),
            ),
            Box::leak(self.data.1.data.data.to_vec().into_boxed_slice()),
            Box::leak(vec![u8::from(self.data.2.data)].into_boxed_slice()),
            Box::leak(
                self.data
                    .3
                    .data
                    .to_le_bytes()
                    .as_slice()
                    .to_owned()
                    .into_boxed_slice(),
            ),
            Box::leak(
                self.data
                    .4
                    .data
                    .to_le_bytes()
                    .as_slice()
                    .to_owned()
                    .into_boxed_slice(),
            ),
        ];

        iterators
            .into_iter()
            .flat_map::<_, fn(&'static [u8]) -> std::iter::Copied<slice::Iter<'static, u8>>>(
                |slice| slice.iter().copied(),
            )
            .enumerate()
    }
}
