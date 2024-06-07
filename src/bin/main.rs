fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_save_face_data_unk_288_bytes() {
        let expected_unk_288_bytes = [
            0x46, 0x41, 0x43, 0x45, 0x04, 0x00, 0x00, 0x00, 0x20, 0x01, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
            0x00, 0x00, 0xCD, 0x6C, 0x00, 0x00, 0x76, 0xA8, 0x6C, 0x58, 0xBC, 0x62, 0x94, 0x4E,
            0xA8, 0x62, 0x44, 0x80, 0x8A, 0x6C, 0x6C, 0x8A, 0x8A, 0xC6, 0x3A, 0x80, 0xB2, 0x00,
            0x80, 0x4E, 0x44, 0x73, 0x00, 0xB2, 0x80, 0x00, 0x76, 0x6C, 0x62, 0x76, 0x80, 0xA8,
            0x62, 0x8C, 0xC6, 0x8A, 0x80, 0xA8, 0x80, 0x76, 0x69, 0xD0, 0x8A, 0x80, 0x80, 0x87,
            0xC6, 0x62, 0xC6, 0x7D, 0x26, 0xBC, 0x55, 0x8A, 0x9E, 0x46, 0x80, 0x00, 0x00, 0x00,
            0x00, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x00, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x8F, 0x67, 0x4F,
            0xA0, 0xFF, 0xD7, 0x50, 0x14, 0x1E, 0x19, 0x00, 0xFF, 0x41, 0x41, 0x00, 0x00, 0x00,
            0x00, 0x64, 0x32, 0x19, 0x00, 0x1E, 0x28, 0x14, 0x1E, 0x14, 0xFF, 0x57, 0x57, 0x80,
            0xD2, 0x80, 0x80, 0x47, 0x25, 0x18, 0x80, 0x01, 0x00, 0x46, 0x30, 0x1D, 0x1A, 0x0F,
            0x05, 0xC8, 0x00, 0x64, 0x64, 0x64, 0xFF, 0xFF, 0xFF, 0x8A, 0x1A, 0x0F, 0x05, 0xC8,
            0x00, 0x64, 0x64, 0x64, 0xFF, 0xFF, 0xFF, 0x8A, 0x46, 0x30, 0x1D, 0x4E, 0x80, 0x50,
            0x46, 0x30, 0x1D, 0x4E, 0x80, 0x50, 0x46, 0x30, 0x1D, 0x4E, 0x80, 0x50, 0x00, 0x00,
            0x00, 0x3C, 0x3C, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let unk_288_bytes =
            er_save_file_readers::models::save_slot::face_data::unk_288_bytes::Unk288Bytes::read_from_file(
                "testdata/vagabond/save_slots/0/face_data/unk_288_bytes.sl2",
            )
            .expect("data should be present");

        for (index, val) in unk_288_bytes.data.data.as_slice().iter().enumerate() {
            assert_eq!(
                val, &expected_unk_288_bytes[index],
                "Failed at index {}, hex: {:02X}\u{2008}",
                index, expected_unk_288_bytes[index]
            )
        }
    }

    #[test]
    fn test_read_save_face_data_unk_bool() {
        let expected_unk_bool = vec![0x01; 2];
        let unk_bool =
            er_save_file_readers::models::save_slot::face_data::unk_bool::UnkBool::read_from_file(
                "testdata/vagabond/save_slots/0/face_data/unk_bool.sl2",
            )
            .expect("data should be present");

        for (index, val) in (if unk_bool.data { 1 as u8 } else { 0 as u8 })
            .to_le_bytes()
            .iter()
            .enumerate()
        {
            assert_eq!(
                val, &expected_unk_bool[index],
                "Failed at index {}, hex: {:02X}\u{2008}",
                index, expected_unk_bool[index]
            )
        }
    }

    #[test]
    fn test_read_save_face_data_unk_i16() {
        let expected_unk_i16 = vec![0x01; 2];
        let unk_i16 =
            er_save_file_readers::models::save_slot::face_data::unk_i16::UnkI16::read_from_file(
                "testdata/vagabond/save_slots/0/face_data/unk_i16.sl2",
            )
            .expect("data should be present");

        for (index, val) in unk_i16.data.to_le_bytes().iter().enumerate() {
            assert_eq!(
                val, &expected_unk_i16[index],
                "Failed at index {}, hex: {:02X}\u{2008}",
                index, expected_unk_i16[index]
            )
        }
    }

    #[test]
    fn test_read_save_face_data_unk_i32() {
        let expected_unk_i32 = vec![0xFF; 4];
        let unk_i32 =
            er_save_file_readers::models::save_slot::face_data::unk_i32::UnkI32::read_from_file(
                "testdata/vagabond/save_slots/0/face_data/unk_i32.sl2",
            )
            .expect("data should be present");

        for (index, val) in unk_i32.data.to_le_bytes().iter().enumerate() {
            assert_eq!(
                val, &expected_unk_i32[index],
                "Failed at index {}, hex: {:02X}\u{2008}",
                index, expected_unk_i32[index]
            )
        }
    }

    #[test]
    fn test_read_save_face_data_unk_i64() {
        let expected_unk_i64 = vec![0xFF; 8];
        let unk_i64 =
            er_save_file_readers::models::save_slot::face_data::unk_i64::UnkI64::read_from_file(
                "testdata/vagabond/save_slots/0/face_data/unk_i64.sl2",
            )
            .expect("data should be present");

        for (index, val) in unk_i64.data.to_le_bytes().iter().enumerate() {
            assert_eq!(
                val, &expected_unk_i64[index],
                "Failed at index {}, hex: {:02X}\u{2008}",
                index, expected_unk_i64[index]
            )
        }
    }

    #[test]
    fn test_read_save_face_data() {
        let expected_face_data = vec![
            0xFF, 0xFF, 0xFF, 0xFF, 0x46, 0x41, 0x43, 0x45, 0x04, 0x00, 0x00, 0x00, 0x20, 0x01,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x03, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0xCD, 0x6C, 0x00, 0x00, 0x76, 0xA8, 0x6C, 0x58,
            0xBC, 0x62, 0x94, 0x4E, 0xA8, 0x62, 0x44, 0x80, 0x8A, 0x6C, 0x6C, 0x8A, 0x8A, 0xC6,
            0x3A, 0x80, 0xB2, 0x00, 0x80, 0x4E, 0x44, 0x73, 0x00, 0xB2, 0x80, 0x00, 0x76, 0x6C,
            0x62, 0x76, 0x80, 0xA8, 0x62, 0x8C, 0xC6, 0x8A, 0x80, 0xA8, 0x80, 0x76, 0x69, 0xD0,
            0x8A, 0x80, 0x80, 0x87, 0xC6, 0x62, 0xC6, 0x7D, 0x26, 0xBC, 0x55, 0x8A, 0x9E, 0x46,
            0x80, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x80,
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x8F, 0x67, 0x4F, 0xA0, 0xFF, 0xD7, 0x50, 0x14, 0x1E, 0x19, 0x00, 0xFF, 0x41,
            0x41, 0x00, 0x00, 0x00, 0x00, 0x64, 0x32, 0x19, 0x00, 0x1E, 0x28, 0x14, 0x1E, 0x14,
            0xFF, 0x57, 0x57, 0x80, 0xD2, 0x80, 0x80, 0x47, 0x25, 0x18, 0x80, 0x01, 0x00, 0x46,
            0x30, 0x1D, 0x1A, 0x0F, 0x05, 0xC8, 0x00, 0x64, 0x64, 0x64, 0xFF, 0xFF, 0xFF, 0x8A,
            0x1A, 0x0F, 0x05, 0xC8, 0x00, 0x64, 0x64, 0x64, 0xFF, 0xFF, 0xFF, 0x8A, 0x46, 0x30,
            0x1D, 0x4E, 0x80, 0x50, 0x46, 0x30, 0x1D, 0x4E, 0x80, 0x50, 0x46, 0x30, 0x1D, 0x4E,
            0x80, 0x50, 0x00, 0x00, 0x00, 0x3C, 0x3C, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01,
            0x01, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];
        assert_eq!(expected_face_data.len(), 303);
        let face_data =
            er_save_file_readers::models::save_slot::face_data::face_data::FaceData::read_from_file(
                "testdata/vagabond/save_slots/0/FaceData.sl2",
            )
            .expect("data should be present");

        for (index, val) in face_data.into_iter().enumerate() {
            assert_eq!(
                val.1, expected_face_data[index],
                "Failed at index {}, hex: {:02X}\u{2008}",
                index, expected_face_data[index]
            )
        }
    }

    #[test]
    fn test_read_save_slot_checksum() {
        let expected_checksum = vec![
            10, 144, 247, 6, 252, 237, 233, 82, 77, 22, 69, 110, 236, 58, 252, 136,
        ];
        let checksum_data =
            er_save_file_readers::models::save_slot::checksum::Checksum::read_from_file(
                "testdata/vagabond/save_slots/0/checksum.sl2",
            )
            .expect("data should be present");

        for (index, &val) in checksum_data.data.iter().enumerate() {
            assert_eq!(val, expected_checksum[index])
        }
    }

    #[test]
    fn test_read_save_slot() {
        let save_slot_data =
            er_save_file_readers::models::save_slot::save_slot::SaveSlot::read_from_file(
                "testdata/vagabond/save_slots/0.sl2",
            )
            .expect("data should be present");
        println!("{:?}", save_slot_data);

        assert_eq!(save_slot_data.checksum.data[0], 10)
    }

    #[test]
    fn test_read_save_slot_unk01() {
        let unk01 = er_save_file_readers::models::save_slot::unk01::Unk01::read_from_file(
            "testdata/vagabond/save_slots/0/unk01.sl2",
        )
        .expect("data should be present");
        println!("{:?}", unk01);

        assert_eq!(unk01.data, 0x00000097)
    }

    #[test]
    fn test_read_save_slot_map_id() {
        let map_id = er_save_file_readers::models::save_slot::map_id::MapID::read_from_file(
            "testdata/vagabond/save_slots/0/MapID.sl2",
        )
        .expect("data should be present");
        println!("{:?}", map_id);

        assert_eq!(map_id.data, 0xA010000)
    }

    #[test]
    fn test_read_save_slot_unk_24_bytes() {
        let map_id =
            er_save_file_readers::models::save_slot::unk_24_bytes::Unk24Bytes::read_from_file(
                "testdata/vagabond/save_slots/0/unk0x18.sl2",
            )
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
        let gaitem_handle_map =
            er_save_file_readers::models::save_slot::gaitem_handle_map::gaitem_handle_map::GAItemHandleMap::read_from_file("testdata/vagabond/save_slots/0/gaitem_handle_map.sl2")
                .expect("data should be present");
        for idx in 0..gaitem_handle_map.gaitem_handles.len() {
            if idx < 397 {
                assert_eq!(
                    gaitem_handle_map.gaitem_handles[idx],
                    er_save_file_readers::models::save_slot::gaitem_handle_map::gaitem_handle::GAItemHandle {
                        ga_item_handle: 0x00000000,
                        item_id: 0xFFFFFFFFu32 as i32
                    },
                    "Failed at index {}",
                    idx
                )
            } else if idx < 400 && idx > 399 {
                assert_eq!(
                    gaitem_handle_map.gaitem_handles[idx],
                    er_save_file_readers::models::save_slot::gaitem_handle_map::gaitem_handle::GAItemHandle {
                        ga_item_handle: 0x00000000,
                        item_id: 0x00000000
                    },
                    "Failed at index {}",
                    idx
                )
            } else if idx > 400 && idx < 419 {
                assert_eq!(
                    gaitem_handle_map.gaitem_handles[idx],
                    er_save_file_readers::models::save_slot::gaitem_handle_map::gaitem_handle::GAItemHandle {
                        ga_item_handle: 0x00000000,
                        item_id: 0xFFFFFFFFu32 as i32
                    },
                    "Failed at index {}",
                    idx
                )
            } else if idx > 419 && idx < 421 {
                assert_eq!(
                    gaitem_handle_map.gaitem_handles[idx],
                    er_save_file_readers::models::save_slot::gaitem_handle_map::gaitem_handle::GAItemHandle {
                        ga_item_handle: 0x00000000,
                        item_id: 0x00000000
                    },
                    "Failed at index {}",
                    idx
                )
            }
        }
    }

    #[test]
    fn test_read_save_slot_player_game_data_unk_bytes() {
        let unk_bytes =
            er_save_file_readers::models::save_slot::player_game_data::unk::Unk::read_from_file(
                "testdata/vagabond/save_slots/0/player_game_data/unk.sl2",
            )
            .expect("data should be present");
        assert_eq!(unk_bytes.data, 0xFFFFFFFFu32 as i32)
    }

    #[test]
    fn test_read_save_slot_player_game_data_unk1_bytes() {
        let unk1_bytes =
            er_save_file_readers::models::save_slot::player_game_data::unk1::Unk1::read_from_file(
                "testdata/vagabond/save_slots/0/player_game_data/unk1.sl2",
            )
            .expect("data should be present");
        assert_eq!(unk1_bytes.data, 0x0i32)
    }

    #[test]
    fn test_read_save_slot_player_game_data_health_bytes() {
        let bytes =
            er_save_file_readers::models::save_slot::player_game_data::health::Health::read_from_file("testdata/vagabond/save_slots/0/player_game_data/health.sl2")
                .expect("data should be present");
        assert_eq!(bytes.data, 0x20A)
    }

    #[test]
    fn test_read_save_slot_player_game_data_max_health_bytes() {
        let bytes = er_save_file_readers::models::save_slot::player_game_data::max_health::MaxHealth::read_from_file(
            "testdata/vagabond/save_slots/0/player_game_data/max_health.sl2",
        )
        .expect("data should be present");
        assert_eq!(bytes.data, 0x20A)
    }

    #[test]
    fn test_read_save_slot_player_game_data_max_base_health_bytes() {
        let bytes = er_save_file_readers::models::save_slot::player_game_data::max_base_health::MaxBaseHealth::read_from_file(
            "testdata/vagabond/save_slots/0/player_game_data/max_base_health.sl2",
        )
        .expect("data should be present");
        assert_eq!(bytes.data, 0x20A)
    }

    #[test]
    fn test_read_save_slot_player_game_data_fp_bytes() {
        let bytes =
            er_save_file_readers::models::save_slot::player_game_data::fp::FP::read_from_file(
                "testdata/vagabond/save_slots/0/player_game_data/fp.sl2",
            )
            .expect("data should be present");
        assert_eq!(bytes.data, 0x4E)
    }

    #[test]
    fn test_read_save_slot_player_game_data_max_fp_bytes() {
        let bytes =
            er_save_file_readers::models::save_slot::player_game_data::max_fp::MaxFP::read_from_file("testdata/vagabond/save_slots/0/player_game_data/max_fp.sl2")
                .expect("data should be present");
        assert_eq!(bytes.data, 0x4E)
    }

    #[test]
    fn test_read_save_slot_player_game_data_max_base_fp_bytes() {
        let bytes = er_save_file_readers::models::save_slot::player_game_data::base_max_fp::BaseMaxFP::read_from_file(
            "testdata/vagabond/save_slots/0/player_game_data/base_max_fp.sl2",
        )
        .expect("data should be present");
        assert_eq!(bytes.data, 0x4E)
    }

    #[test]
    fn test_read_save_slot_player_game_data_unk2_bytes() {
        let bytes =
            er_save_file_readers::models::save_slot::player_game_data::unk2::Unk2::read_from_file(
                "testdata/vagabond/save_slots/0/player_game_data/unk2.sl2",
            )
            .expect("data should be present");
        assert_eq!(bytes.data, 0x0)
    }
}
