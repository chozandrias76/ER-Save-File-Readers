use std::io::{self, Read, Seek};

use crate::traits::{binary_readable::BinaryReadable, validate::Validate};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct GAItemHandle {
    pub ga_item: i32,
    pub item_id: i32,
}

impl Default for GAItemHandle {
    fn default() -> Self {
        GAItemHandle {
            ga_item: i32::default(),
            item_id: i32::default(),
        }
    }
}

impl BinaryReadable for GAItemHandle {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mut ga_item_handle_bytes = [0u8; 4];
        let mut item_id_bytes = [0u8; 4];

        // Read the ga_item_handle field (i32)
        reader.read_exact(&mut ga_item_handle_bytes)?;
        let ga_item_handle = i32::from_le_bytes(ga_item_handle_bytes);

        // Read the item_id field (i32)
        reader.read_exact(&mut item_id_bytes)?;
        let item_id = i32::from_le_bytes(item_id_bytes);

        Ok(GAItemHandle {
            ga_item: ga_item_handle,
            item_id,
        })
    }
}

impl Validate for GAItemHandle {
    fn validate(&self) -> bool {
        self.ga_item.to_le_bytes().len() == 4 && self.item_id.to_le_bytes().len() == 4
    }
}
