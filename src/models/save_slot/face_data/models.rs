pub mod accessory;
pub mod beard;
pub mod decal;
pub mod eye;
pub mod eyebrow;
pub mod eyelash;
pub mod face;
pub mod hair;

use std::io::{self, Read, Seek};

use crate::{models::shared::u8_reader::U8Reader, traits::binary_readable::BinaryReadable};

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Model {
    pub data: (u8, u8, u8, u8),
}

impl BinaryReadable for Model {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        Ok(Model {
            data: {
                let mut arr = [0; 4];
                for (_i, item) in arr.iter_mut().enumerate() {
                    *item = U8Reader::read(reader).unwrap().data;
                }
                arr.into()
            },
        })
    }
}

impl Default for Model {
    fn default() -> Self {
        Model {
            data: [U8Reader::default().data; 4].into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::binary_readable::BinaryReadable;

    #[test]
    fn test_read_face_model() {
        let mut reader = std::io::Cursor::new(vec![0x00, 0x00, 0x00, 0x00]);
        match crate::models::save_slot::face_data::models::face::Face::read(&mut reader) {
            Ok(parent) => {
                assert_eq!(parent.model.data, (0, 0, 0, 0));
            }
            Err(e) => {
                panic!("Error reading model: {:?}", e);
            }
        };
    }

    #[test]
    fn test_read_hair_model() {
        let mut reader = std::io::Cursor::new(vec![0x09, 0x00, 0x00, 0x00]);
        match crate::models::save_slot::face_data::models::hair::Hair::read(&mut reader) {
            Ok(parent) => {
                assert_eq!(parent.model.data, (9, 0, 0, 0));
            }
            Err(e) => {
                panic!("Error reading model: {:?}", e);
            }
        };
    }

    #[test]
    fn test_read_eye_model() {
        let mut reader = std::io::Cursor::new(vec![0x00, 0x00, 0x00, 0x00]);
        match crate::models::save_slot::face_data::models::eye::Eye::read(&mut reader) {
            Ok(parent) => {
                assert_eq!(parent.model.data, (0, 0, 0, 0));
            }
            Err(e) => {
                panic!("Error reading model: {:?}", e);
            }
        };
    }

    #[test]
    fn test_read_eyebrow_model() {
        let mut reader = std::io::Cursor::new(vec![0x03, 0x00, 0x00, 0x00]);
        match crate::models::save_slot::face_data::models::eyebrow::Eyebrow::read(&mut reader) {
            Ok(parent) => {
                assert_eq!(parent.model.data, (3, 0, 0, 0));
            }
            Err(e) => {
                panic!("Error reading model: {:?}", e);
            }
        };
    }

    #[test]
    fn test_read_beard_model() {
        let mut reader = std::io::Cursor::new(vec![0x01, 0x00, 0x00, 0x00]);
        match crate::models::save_slot::face_data::models::beard::Beard::read(&mut reader) {
            Ok(parent) => {
                assert_eq!(parent.model.data, (1, 0, 0, 0));
            }
            Err(e) => {
                panic!("Error reading model: {:?}", e);
            }
        };
    }

    #[test]
    fn test_read_accessory_model() {
        let mut reader = std::io::Cursor::new(vec![0x00, 0x00, 0x00, 0x00]);
        match crate::models::save_slot::face_data::models::accessory::Accessory::read(&mut reader) {
            Ok(parent) => {
                assert_eq!(parent.model.data, (0, 0, 0, 0));
            }
            Err(e) => {
                panic!("Error reading model: {:?}", e);
            }
        };
    }

    #[test]
    fn test_read_decal_model() {
        let mut reader = std::io::Cursor::new(vec![0x00, 0x00, 0x00, 0x00]);
        match crate::models::save_slot::face_data::models::decal::Decal::read(&mut reader) {
            Ok(parent) => {
                assert_eq!(parent.model.data, (0, 0, 0, 0));
            }
            Err(e) => {
                panic!("Error reading model: {:?}", e);
            }
        };
    }

    #[test]
    fn test_read_eyelash_model() {
        let mut reader = std::io::Cursor::new(vec![0x02, 0x00, 0x00, 0x00]);
        match crate::models::save_slot::face_data::models::eyelash::Eyelash::read(&mut reader) {
            Ok(parent) => {
                assert_eq!(parent.model.data, (2, 0, 0, 0));
            }
            Err(e) => {
                panic!("Error reading model: {:?}", e);
            }
        };
    }
}
