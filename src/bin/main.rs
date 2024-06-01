fn main() {}

#[cfg(test)]
mod tests {
    use er_save_file_readers::models::save_slot::checksum::Checksum;
    use er_save_file_readers::models::save_slot::gaitem_handle_map::gaitem_handle::GAItemHandle;
    use er_save_file_readers::models::save_slot::gaitem_handle_map::gaitem_handle_map::GAItemHandleMap;
    use er_save_file_readers::models::save_slot::map_id::MapID;
    use er_save_file_readers::models::save_slot::save_slot::SaveSlot;
    use er_save_file_readers::models::save_slot::unk01::Unk01;
    use er_save_file_readers::models::save_slot::unk_24_bytes::Unk24Bytes;
    use er_save_file_readers::traits::binary_readable::BinaryReadable;
    use std::fs::File;
    use std::io::{self, BufReader};

    fn read_from_file<T: BinaryReadable>(path: &str) -> io::Result<T> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        T::read(&mut reader)
    }

    // Specific functions using the generic function
    fn read_checksum_from_file(path: &str) -> io::Result<Checksum> {
        read_from_file(path)
    }

    fn read_save_slot_from_file(path: &str) -> io::Result<SaveSlot> {
        read_from_file(path)
    }

    fn read_unk01_from_file(path: &str) -> io::Result<Unk01> {
        read_from_file(path)
    }

    fn read_map_id_from_file(path: &str) -> io::Result<MapID> {
        read_from_file(path)
    }

    fn read_gaitem_handle_map_from_file(path: &str) -> io::Result<GAItemHandleMap> {
        read_from_file(path)
    }

    fn read_unk_24_from_file(path: &str) -> io::Result<Unk24Bytes> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Unk24Bytes::read(&mut reader)
    }

    #[test]
    fn test_read_save_slot_checksum() {
        let expected_checksum = vec![
            10, 144, 247, 6, 252, 237, 233, 82, 77, 22, 69, 110, 236, 58, 252, 136,
        ];
        let checksum_data = read_checksum_from_file("testdata/vagabond/save_slots/0/checksum.sl2")
            .expect("data should be present");

        for (index, &val) in checksum_data.data.iter().enumerate() {
            assert_eq!(val, expected_checksum[index])
        }
    }

    #[test]
    fn test_read_save_slot() {
        let save_slot_data = read_save_slot_from_file("testdata/vagabond/save_slots/0.sl2")
            .expect("data should be present");
        println!("{:?}", save_slot_data);

        assert_eq!(save_slot_data.checksum.data[0], 10)
    }

    #[test]
    fn test_read_save_slot_unk01() {
        let unk01 = read_unk01_from_file("testdata/vagabond/save_slots/0/unk01.sl2")
            .expect("data should be present");
        println!("{:?}", unk01);

        assert_eq!(unk01.data, 0x00000097)
    }

    #[test]
    fn test_read_save_slot_map_id() {
        let map_id = read_map_id_from_file("testdata/vagabond/save_slots/0/MapID.sl2")
            .expect("data should be present");
        println!("{:?}", map_id);

        assert_eq!(map_id.data, 0xA010000)
    }

    #[test]
    fn test_read_save_slot_unk_24_bytes() {
        let map_id = read_unk_24_from_file("testdata/vagabond/save_slots/0/unk0x18.sl2")
            .expect("data should be present");
        println!("{:?}", map_id);

        assert_eq!(
            map_id.data.data,
            vec![
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x6e, 0x85, 0x11, 0x77, 0x2e,
                0xba, 0x79, 0xba, 0x99, 0xa9, 0x17, 0x53, 0x68, 0x01, 0x37
            ]
        )
    }

    #[test]
    fn test_read_save_slot_ga_item_handle_map_bytes() {
        let gaitem_handle_map: GAItemHandleMap = read_gaitem_handle_map_from_file(
            "testdata/vagabond/save_slots/0/gaitem_handle_map.sl2",
        )
        .expect("data should be present");
        for idx in 0..gaitem_handle_map.gaitem_handles.len() {
            if idx < 397 {
                assert_eq!(
                    gaitem_handle_map.gaitem_handles[idx],
                    GAItemHandle {
                        ga_item_handle: 0x00000000,
                        item_id: 0xFFFFFFFFu32 as i32
                    },
                    "Failed at index {}",
                    idx
                )
            } else if idx < 400 && idx > 399 {
                assert_eq!(
                    gaitem_handle_map.gaitem_handles[idx],
                    GAItemHandle {
                        ga_item_handle: 0x00000000,
                        item_id: 0x00000000
                    },
                    "Failed at index {}",
                    idx
                )
            } else if idx > 400 && idx < 419 {
                assert_eq!(
                    gaitem_handle_map.gaitem_handles[idx],
                    GAItemHandle {
                        ga_item_handle: 0x00000000,
                        item_id: 0xFFFFFFFFu32 as i32
                    },
                    "Failed at index {}",
                    idx
                )
            } else if idx > 419 && idx < 421 {
                assert_eq!(
                    gaitem_handle_map.gaitem_handles[idx],
                    GAItemHandle {
                        ga_item_handle: 0x00000000,
                        item_id: 0x00000000
                    },
                    "Failed at index {}",
                    idx
                )
            }
        }
    }
}
