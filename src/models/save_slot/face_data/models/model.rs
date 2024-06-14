use std::io::{self, Read, Seek};

use crate::impl_byte_array_readable;
use crate::models::shared::byte_array_reader::ByteArray;

impl_byte_array_readable!(Model, 4);


#[cfg(test)]
mod tests {
    use crate::traits::binary_readable::BinaryReadable;

    #[test]
    fn test_read_face_model() {
        let mut reader = std::io::Cursor::new(vec![0x00, 0x00, 0x00, 0x00]);
        match crate::models::save_slot::face_data::models::face::Face::read(&mut reader) {
            Ok(parent) => {
                assert_eq!(parent.model.data.data, vec![0; 4]);
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
                assert_eq!(parent.model.data.data, [9, 0, 0, 0]);
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
                assert_eq!(parent.model.data.data, [0, 0, 0, 0]);
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
                assert_eq!(parent.model.data.data, [3, 0, 0, 0]);
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
                assert_eq!(parent.model.data.data, [1, 0, 0, 0]);
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
                assert_eq!(parent.model.data.data, [0, 0, 0, 0]);
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
                assert_eq!(parent.model.data.data, [0, 0, 0, 0]);
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
                assert_eq!(parent.model.data.data, [2, 0, 0, 0]);
            }
            Err(e) => {
                panic!("Error reading model: {:?}", e);
            }
        };
    }
}
