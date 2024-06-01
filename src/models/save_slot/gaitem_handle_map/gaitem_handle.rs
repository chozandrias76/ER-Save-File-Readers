use std::io::{self, Read, Seek};

use crate::traits::binary_readable::BinaryReadable;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
pub struct GAItemHandle {
    pub ga_item_handle: i32,
    pub item_id: i32,
}

impl Default for GAItemHandle {
    fn default() -> Self {
        GAItemHandle {
            ga_item_handle: Default::default(),
            item_id: Default::default(),
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
        ga_item_handle,
        item_id,
    })
}
}