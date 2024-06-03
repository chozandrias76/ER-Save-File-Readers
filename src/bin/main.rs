fn main() {}

#[cfg(test)]
mod tests {
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
