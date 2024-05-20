use std::{
    fmt,
    io::{self, Read, Seek}, mem,
};

use crate::traits::binary_readable::BinaryReadable;

use super::checksum::Checksum;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct SaveSlot {
    pub checksum: Checksum,
}

impl SaveSlot {
  pub fn length(&self) -> usize { 
    // let base_size = mem::size_of::<Checksum>();

    2621456// - base_size
  }
}

impl Default for SaveSlot {
    fn default() -> Self {
        Self {
            checksum: Default::default(),
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
        Ok(Self {
            checksum: Checksum {
                data: *checksum_data,
            },
        })
    }
}
