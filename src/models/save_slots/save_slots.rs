use std::io::{self, Read, Seek};

use crate::{models::save_slot::save_slot::SaveSlot, traits::binary_readable::BinaryReadable};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SaveSlots {
    pub data: Vec<SaveSlot>,
}

impl Default for SaveSlots {
    fn default() -> Self {
        Self {
            data: vec![SaveSlot::default(); 10],
        }
    }
}

impl BinaryReadable for SaveSlots {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let save_slot_data = Self::default();
        let save_slot0 = SaveSlot::read(reader)?;
        reader.seek(io::SeekFrom::Current(
            save_slot0.length().try_into().unwrap(),
        )).expect("OK");
        Ok(save_slot_data)
    }
}
