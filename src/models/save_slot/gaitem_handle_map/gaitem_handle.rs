use std::io::{self, Read, Seek};

use crate::traits::{binary_readable::BinaryReadable, validate::Validate};

use super::ga_item::GaItem;
use super::item_id::ItemId;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GAItemHandle {
    pub ga_item: GaItem,
    pub item_id: ItemId,
}

impl Default for GAItemHandle {
    fn default() -> Self {
        GAItemHandle {
            ga_item: GaItem::default(),
            item_id: ItemId::default(),
        }
    }
}

impl PartialEq for GAItemHandle {
    fn eq(&self, other: &Self) -> bool {
        self.ga_item == other.ga_item && self.item_id == other.item_id
    }
}

impl BinaryReadable for GAItemHandle {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let ga_item = GaItem::read(reader)?;
        let item_id = ItemId::read(reader)?;

        Ok(GAItemHandle { ga_item, item_id })
    }
}

impl Validate for GAItemHandle {
    fn validate(&self) -> bool {
        self.ga_item.validate() && self.item_id.validate()
    }
}
