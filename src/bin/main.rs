fn main() {}

#[cfg(test)]
mod tests {
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
        let mut expected_face_data: Vec<u8> = Vec::new();
        let initial_data = [
            0xFF, 0xFF, 0xFF, 0xFF, 0x46, 0x41, 0x43, 0x45, 0x04, 0x00, 0x00, 0x00, 0x20, 0x01,
            0x00, 0x00,
        ];
        let face_model = [0x00, 0x00, 0x00, 0x00];
        let hair_model = [0x09, 0x00, 0x00, 0x00];
        let eye_model = [0x00, 0x00, 0x00, 0x00];
        let eyebrow_model = [0x03, 0x00, 0x00, 0x00];
        let beard_model = [0x01, 0x00, 0x00, 0x00];
        let acc_model = [0x00, 0x00, 0x00, 0x00];
        let decal_model = [0x00, 0x00, 0x00, 0x00];
        let eyelash_model = [0x02, 0x00, 0x00, 0x00];
        let apparent_age = [0xCD];
        let facial_aesthetic = [0x6C];
        let form_emphasis = [0x00];
        let numen = [0x00];
        let brow_ridge_height = [0x76];
        let inner_brow_height = [0xA8];
        let outer_brow_ridge = [0x6C];
        let cheekbone_height = [0x58];
        let cheekbone_depth = [0xBC];
        let cheekbone_width = [0x62];
        let cheekbone_protrusion = [0x94];
        let cheeks_morph = [0x4E];
        let chin_tip_position = [0xA8];
        let chin_length = [0x62];
        let chin_protrusion = [0x44];
        let chin_depth = [0x80];
        let chin_size = [0x8A];
        let chin_height = [0x6C];
        let chin_width = [0x6C];
        let eye_position = [0x8A];
        let eye_size = [0x8A];
        let eye_slant = [0xC6];
        let eye_spacing = [0x3A];
        let nose_size = [0x80];
        let nose_to_forehead_ratio = [0xB2];
        let unimplemented_1 = [0x00];
        let face_protrusion = [0x80];
        let vert_face_ratio = [0x4E];
        let facial_feature_slant = [0x44];
        let horizontal_face_ratio = [0x73];
        let unimplemented_2 = [0x00];
        let forehead_depth = [0xB2];
        let forehead_protrusion = [0x80];
        let unimplemented_3 = [0x00];
        let jaw_protrusion = [0x76];
        let jaw_width = [0x6C];
        let lower_jaw = [0x62];
        let jaw_contour = [0x76];
        let lip_shape = [0x80];
        let lip_size = [0xA8];
        let lip_fullness = [0x62];
        let mouth_expression = [0x8C];
        let lip_protrusion = [0xC6];
        let lip_thickness = [0x8A];
        let mouth_protrusion = [0x80];
        let mouth_slant = [0xA8];
        let mouth_occlusion = [0x80];
        let mouth_position = [0x76];
        let mouth_width = [0x69];
        let mouth_to_chin_distance = [0xD0];
        let nose_ridge_depth = [0x8A];
        let nose_ridge_length = [0x80];
        let nose_position = [0x80];
        let nose_tip_height = [0x87];
        let nostril_slant = [0xC6];
        let nostril_size = [0x62];
        let nostril_width = [0xC6];
        let nose_protrusion = [0x7D];
        let nose_bridge_height = [0x26];
        let nose_bridge_protrusion_1 = [0xBC];
        let nose_bridge_protrusion_2 = [0x55];
        let nose_bridge_with = [0x8A];
        let nose_height = [0x9E];
        let nose_slant = [0x46];
        let unimplemented_4: [u8; 64] = [
            0x80, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x80,
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
            0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00,
        ];

        let body_scale_head = [0x80];
        let body_scale_breast = [0x80];
        let body_scale_abdomen = [0x80];
        let body_scale_arm_right = [0x80];
        let body_scale_leg_right = [0x80];
        let body_scale_arm_left = [0x80];
        let body_scale_leg_left = [0x80];
        let skin_color_r = [0x8F];
        let skin_color_g = [0x67];
        let skin_color_b = [0x4F];
        let skin_color_a = [0xA0];
        let skin_pores = [0xFF];
        let beard_stubble = [0xD7];
        let skin_dark_circle = [0x50];
        let skin_dark_circle_color_r = [0x14];
        let skin_dark_circle_color_g = [0x1E];
        let skin_dark_circle_color_b = [0x19];
        let cheeks = [0x00];
        let cheeks_color_r = [0xFF];
        let cheeks_color_g = [0x41];
        let cheeks_color_b = [0x41];
        let eyeliner = [0x00];
        let eyeliner_color_r = [0x00];
        let eyeliner_color_g = [0x00];
        let eyeliner_color_b = [0x00];
        let eyeshadow_lower = [0x64];
        let eyeshadow_lower_color_r = [0x32];
        let eyeshadow_lower_color_g = [0x19];
        let eyeshadow_lower_color_b = [0x00];
        let eyeshadow_upper = [0x1E];
        let eyeshadow_upper_color_r = [0x28];
        let eyeshadow_upper_color_g = [0x14];
        let eyeshadow_upper_color_b = [0x1E];
        let lipstick = [0x14];
        let lipstick_color_r = [0xFF];
        let lipstick_color_g = [0x57];
        let lipstick_color_b = [0x57];
        let decal_position_x = [0x80];
        let decal_position_y = [0xD2];
        let decal_angle = [0x80];
        let decal_scale = [0x80];
        let decal_color_r = [0x47];
        let decal_color_g = [0x25];
        let decal_color_b = [0x18];
        let unimplemented_5 = [0x80];
        let decal_flip = [0x01];
        let body_hair = [0x00];
        let body_hair_color_r = [0x46];
        let body_hair_color_g = [0x30];
        let body_hair_color_b = [0x1D];
        let eye_right_color_r = [0x1A];
        let eye_right_color_g = [0x0F];
        let eye_right_color_b = [0x05];
        let eye_right_iris_size = [0xC8];
        let eye_right_clouding = [0x00];
        let eye_right_clouding_r = [0x64];
        let eye_right_clouding_g = [0x64];
        let eye_right_clouding_b = [0x64];
        let eye_right_sclera_r = [0xFF];
        let eye_right_sclera_g = [0xFF];
        let eye_right_sclera_b = [0xFF];
        let eye_right_position = [0x8A];
        let eye_left_color_r = [0x1A];
        let eye_left_color_g = [0x0F];
        let eye_left_color_b = [0x05];
        let eye_left_iris_size = [0xC8];
        let eye_left_clouding = [0x00];
        let eye_left_clouding_r = [0x64];
        let eye_left_clouding_g = [0x64];
        let eye_left_clouding_b = [0x64];
        let eye_left_sclera_r = [0xFF];
        let eye_left_sclera_g = [0xFF];
        let eye_left_sclera_b = [0xFF];
        let eye_left_position = [0x8A];
        let hair_color_r = [0x46];
        let hair_color_g = [0x30];
        let hair_color_b = [0x1D];
        let hair_color_a = [0x4E];
        let hair_color_root_darkness = [0x80];
        let hair_color_white = [0x50];
        let beard_color_r = [0x46];
        let beard_color_g = [0x30];
        let beard_color_b = [0x1D];
        let beard_color_a = [0x4E];
        let beard_color_root_darkness = [0x80];
        let beard_color_white = [0x50];
        let eyebrow_color_r = [0x46];
        let eyebrow_color_g = [0x30];
        let eyebrow_color_b = [0x1D];
        let eyebrow_color_a = [0x4E];
        let eyebrow_color_root_darkness = [0x80];
        let eyebrow_color_white = [0x50];
        let eyelash_color_r = [0x00];
        let eyelash_color_g = [0x00];
        let eyelash_color_b = [0x00];
        let accessories_color_r = [0x3C];
        let accessories_color_g = [0x3C];
        let accessories_color_b = [0x3C];
        let frenzied_flame = [0x00];
        let unimplemented_6 = [0x00];
        let unimplemented_7 = [0x00];
        let unimplemented_8 = [0x00];
        let unimplemented_9 = [0x00];
        let unimplemented_10 = [0x00];
        let unimplemented_11 = [0x00];
        let unimplemented_12 = [0x00];

        let end_data = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x01, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        ];
        // Unknown
        expected_face_data.extend_from_slice(&initial_data);
        // Models
        expected_face_data.extend_from_slice(&face_model);
        expected_face_data.extend_from_slice(&hair_model);
        expected_face_data.extend_from_slice(&eye_model);
        expected_face_data.extend_from_slice(&eyebrow_model);
        expected_face_data.extend_from_slice(&beard_model);
        expected_face_data.extend_from_slice(&acc_model);
        expected_face_data.extend_from_slice(&decal_model);
        expected_face_data.extend_from_slice(&eyelash_model);
        // Other
        expected_face_data.extend_from_slice(&apparent_age);
        expected_face_data.extend_from_slice(&facial_aesthetic);
        expected_face_data.extend_from_slice(&form_emphasis);
        expected_face_data.extend_from_slice(&numen);
        expected_face_data.extend_from_slice(&brow_ridge_height);
        expected_face_data.extend_from_slice(&inner_brow_height);
        expected_face_data.extend_from_slice(&outer_brow_ridge);
        expected_face_data.extend_from_slice(&cheekbone_height);
        expected_face_data.extend_from_slice(&cheekbone_depth);
        expected_face_data.extend_from_slice(&cheekbone_width);
        expected_face_data.extend_from_slice(&cheekbone_protrusion);
        expected_face_data.extend_from_slice(&cheeks_morph);
        expected_face_data.extend_from_slice(&chin_tip_position);
        expected_face_data.extend_from_slice(&chin_length);
        expected_face_data.extend_from_slice(&chin_protrusion);
        expected_face_data.extend_from_slice(&chin_depth);
        expected_face_data.extend_from_slice(&chin_size);
        expected_face_data.extend_from_slice(&chin_height);
        expected_face_data.extend_from_slice(&chin_width);
        expected_face_data.extend_from_slice(&eye_position);
        expected_face_data.extend_from_slice(&eye_size);
        expected_face_data.extend_from_slice(&eye_slant);
        expected_face_data.extend_from_slice(&eye_spacing);
        expected_face_data.extend_from_slice(&nose_size);
        expected_face_data.extend_from_slice(&nose_to_forehead_ratio);
        expected_face_data.extend_from_slice(&unimplemented_1);
        expected_face_data.extend_from_slice(&face_protrusion);
        expected_face_data.extend_from_slice(&vert_face_ratio);
        expected_face_data.extend_from_slice(&facial_feature_slant);
        expected_face_data.extend_from_slice(&horizontal_face_ratio);
        expected_face_data.extend_from_slice(&unimplemented_2);
        expected_face_data.extend_from_slice(&forehead_depth);
        expected_face_data.extend_from_slice(&forehead_protrusion);
        expected_face_data.extend_from_slice(&unimplemented_3);
        expected_face_data.extend_from_slice(&jaw_protrusion);
        expected_face_data.extend_from_slice(&jaw_width);
        expected_face_data.extend_from_slice(&lower_jaw);
        expected_face_data.extend_from_slice(&jaw_contour);
        expected_face_data.extend_from_slice(&lip_shape);
        expected_face_data.extend_from_slice(&lip_size);
        expected_face_data.extend_from_slice(&lip_fullness);
        expected_face_data.extend_from_slice(&mouth_expression);
        expected_face_data.extend_from_slice(&lip_protrusion);
        expected_face_data.extend_from_slice(&lip_thickness);
        expected_face_data.extend_from_slice(&mouth_protrusion);
        expected_face_data.extend_from_slice(&mouth_slant);
        expected_face_data.extend_from_slice(&mouth_occlusion);
        expected_face_data.extend_from_slice(&mouth_position);
        expected_face_data.extend_from_slice(&mouth_width);
        expected_face_data.extend_from_slice(&mouth_to_chin_distance);
        expected_face_data.extend_from_slice(&nose_ridge_depth);
        expected_face_data.extend_from_slice(&nose_ridge_length);
        expected_face_data.extend_from_slice(&nose_position);
        expected_face_data.extend_from_slice(&nose_tip_height);
        expected_face_data.extend_from_slice(&nostril_slant);
        expected_face_data.extend_from_slice(&nostril_size);
        expected_face_data.extend_from_slice(&nostril_width);
        expected_face_data.extend_from_slice(&nose_protrusion);
        expected_face_data.extend_from_slice(&nose_bridge_height);
        expected_face_data.extend_from_slice(&nose_bridge_protrusion_1);
        expected_face_data.extend_from_slice(&nose_bridge_protrusion_2);
        expected_face_data.extend_from_slice(&nose_bridge_with);
        expected_face_data.extend_from_slice(&nose_height);
        expected_face_data.extend_from_slice(&nose_slant);
        expected_face_data.extend_from_slice(&unimplemented_4);
        expected_face_data.extend_from_slice(&body_scale_head);
        expected_face_data.extend_from_slice(&body_scale_breast);
        expected_face_data.extend_from_slice(&body_scale_abdomen);
        expected_face_data.extend_from_slice(&body_scale_arm_right);
        expected_face_data.extend_from_slice(&body_scale_leg_right);
        expected_face_data.extend_from_slice(&body_scale_arm_left);
        expected_face_data.extend_from_slice(&body_scale_leg_left);
        expected_face_data.extend_from_slice(&skin_color_r);
        expected_face_data.extend_from_slice(&skin_color_g);
        expected_face_data.extend_from_slice(&skin_color_b);
        expected_face_data.extend_from_slice(&skin_color_a);
        expected_face_data.extend_from_slice(&skin_pores);
        expected_face_data.extend_from_slice(&beard_stubble);
        expected_face_data.extend_from_slice(&skin_dark_circle);
        expected_face_data.extend_from_slice(&skin_dark_circle_color_r);
        expected_face_data.extend_from_slice(&skin_dark_circle_color_g);
        expected_face_data.extend_from_slice(&skin_dark_circle_color_b);
        expected_face_data.extend_from_slice(&cheeks);
        expected_face_data.extend_from_slice(&cheeks_color_r);
        expected_face_data.extend_from_slice(&cheeks_color_g);
        expected_face_data.extend_from_slice(&cheeks_color_b);
        expected_face_data.extend_from_slice(&eyeliner);
        expected_face_data.extend_from_slice(&eyeliner_color_r);
        expected_face_data.extend_from_slice(&eyeliner_color_g);
        expected_face_data.extend_from_slice(&eyeliner_color_b);
        expected_face_data.extend_from_slice(&eyeshadow_lower);
        expected_face_data.extend_from_slice(&eyeshadow_lower_color_r);
        expected_face_data.extend_from_slice(&eyeshadow_lower_color_g);
        expected_face_data.extend_from_slice(&eyeshadow_lower_color_b);
        expected_face_data.extend_from_slice(&eyeshadow_upper);
        expected_face_data.extend_from_slice(&eyeshadow_upper_color_r);
        expected_face_data.extend_from_slice(&eyeshadow_upper_color_g);
        expected_face_data.extend_from_slice(&eyeshadow_upper_color_b);
        expected_face_data.extend_from_slice(&lipstick);
        expected_face_data.extend_from_slice(&lipstick_color_r);
        expected_face_data.extend_from_slice(&lipstick_color_g);
        expected_face_data.extend_from_slice(&lipstick_color_b);
        expected_face_data.extend_from_slice(&decal_position_x);
        expected_face_data.extend_from_slice(&decal_position_y);
        expected_face_data.extend_from_slice(&decal_angle);
        expected_face_data.extend_from_slice(&decal_scale);
        expected_face_data.extend_from_slice(&decal_color_r);
        expected_face_data.extend_from_slice(&decal_color_g);
        expected_face_data.extend_from_slice(&decal_color_b);
        expected_face_data.extend_from_slice(&unimplemented_5);
        expected_face_data.extend_from_slice(&decal_flip);
        expected_face_data.extend_from_slice(&body_hair);
        expected_face_data.extend_from_slice(&body_hair_color_r);
        expected_face_data.extend_from_slice(&body_hair_color_g);
        expected_face_data.extend_from_slice(&body_hair_color_b);
        expected_face_data.extend_from_slice(&eye_right_color_r);
        expected_face_data.extend_from_slice(&eye_right_color_g);
        expected_face_data.extend_from_slice(&eye_right_color_b);
        expected_face_data.extend_from_slice(&eye_right_iris_size);
        expected_face_data.extend_from_slice(&eye_right_clouding);
        expected_face_data.extend_from_slice(&eye_right_clouding_r);
        expected_face_data.extend_from_slice(&eye_right_clouding_g);
        expected_face_data.extend_from_slice(&eye_right_clouding_b);
        expected_face_data.extend_from_slice(&eye_right_sclera_r);
        expected_face_data.extend_from_slice(&eye_right_sclera_g);
        expected_face_data.extend_from_slice(&eye_right_sclera_b);
        expected_face_data.extend_from_slice(&eye_right_position);
        expected_face_data.extend_from_slice(&eye_left_color_r);
        expected_face_data.extend_from_slice(&eye_left_color_g);
        expected_face_data.extend_from_slice(&eye_left_color_b);
        expected_face_data.extend_from_slice(&eye_left_iris_size);
        expected_face_data.extend_from_slice(&eye_left_clouding);
        expected_face_data.extend_from_slice(&eye_left_clouding_r);
        expected_face_data.extend_from_slice(&eye_left_clouding_g);
        expected_face_data.extend_from_slice(&eye_left_clouding_b);
        expected_face_data.extend_from_slice(&eye_left_sclera_r);
        expected_face_data.extend_from_slice(&eye_left_sclera_g);
        expected_face_data.extend_from_slice(&eye_left_sclera_b);
        expected_face_data.extend_from_slice(&eye_left_position);
        expected_face_data.extend_from_slice(&hair_color_r);
        expected_face_data.extend_from_slice(&hair_color_g);
        expected_face_data.extend_from_slice(&hair_color_b);
        expected_face_data.extend_from_slice(&hair_color_a);
        expected_face_data.extend_from_slice(&hair_color_root_darkness);
        expected_face_data.extend_from_slice(&hair_color_white);
        expected_face_data.extend_from_slice(&beard_color_r);
        expected_face_data.extend_from_slice(&beard_color_g);
        expected_face_data.extend_from_slice(&beard_color_b);
        expected_face_data.extend_from_slice(&beard_color_a);
        expected_face_data.extend_from_slice(&beard_color_root_darkness);
        expected_face_data.extend_from_slice(&beard_color_white);
        expected_face_data.extend_from_slice(&eyebrow_color_r);
        expected_face_data.extend_from_slice(&eyebrow_color_g);
        expected_face_data.extend_from_slice(&eyebrow_color_b);
        expected_face_data.extend_from_slice(&eyebrow_color_a);
        expected_face_data.extend_from_slice(&eyebrow_color_root_darkness);
        expected_face_data.extend_from_slice(&eyebrow_color_white);
        expected_face_data.extend_from_slice(&eyelash_color_r);
        expected_face_data.extend_from_slice(&eyelash_color_g);
        expected_face_data.extend_from_slice(&eyelash_color_b);
        expected_face_data.extend_from_slice(&accessories_color_r);
        expected_face_data.extend_from_slice(&accessories_color_g);
        expected_face_data.extend_from_slice(&accessories_color_b);
        expected_face_data.extend_from_slice(&frenzied_flame);
        expected_face_data.extend_from_slice(&unimplemented_6);
        expected_face_data.extend_from_slice(&unimplemented_7);
        expected_face_data.extend_from_slice(&unimplemented_8);
        expected_face_data.extend_from_slice(&unimplemented_9);
        expected_face_data.extend_from_slice(&unimplemented_10);
        expected_face_data.extend_from_slice(&unimplemented_11);
        expected_face_data.extend_from_slice(&unimplemented_12);

        //Unknown
        expected_face_data.extend_from_slice(&end_data);
        assert_eq!(expected_face_data.len(), 303);
        let face_data =
            er_save_file_readers::models::save_slot::face_data::face_data::FaceData::read_from_file(
                "testdata/vagabond/save_slots/0/FaceData.sl2",
            )
            .expect("data should be present");

        for (index, val) in face_data.data.data.into_iter().enumerate() {
            assert_eq!(
                val, expected_face_data[index],
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
