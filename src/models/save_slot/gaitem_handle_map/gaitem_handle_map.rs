use std::io::{self, Read, Seek};

use crate::traits::{binary_readable::BinaryReadable, validate::Validate};

use super::gaitem_handle::GAItemHandle;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct GAItemHandleMap {
    pub gaitem_handles: Vec<GAItemHandle>,
    length: usize,
}

impl BinaryReadable for GAItemHandleMap {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let length = 5120;
        let mut gaitem_handles = Vec::with_capacity(length);

        for idx in 0..length {
            println!("Reading GAItemHandleMap[{}]", idx);
            let handle = GAItemHandle::read(reader)?;
            gaitem_handles.push(handle);
        }

        Ok(GAItemHandleMap { gaitem_handles, length })
    }
}

impl Validate for GAItemHandleMap {
    fn validate(&self) -> bool {
        self.gaitem_handles.iter().all(|f| f.validate()) &&
        self.gaitem_handles.len() == self.length
    }
}
