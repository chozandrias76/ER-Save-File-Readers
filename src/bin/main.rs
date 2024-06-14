fn main() {}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use er_save_file_readers::{
        models::save_slot::face_data::{
            attributes::{
                accessories_color_b::AccessoriesColorB, accessories_color_g::AccessoriesColorG,
                accessories_color_r::AccessoriesColorR, apparent_age::ApparentAge,
                beard_color_a::BeardColorA, beard_color_b::BeardColorB, beard_color_g::BeardColorG,
                beard_color_r::BeardColorR, beard_color_root_darkness::BeardColorRootDarkness,
                beard_color_white::BeardColorWhite, beard_stubble::BeardStubble,
                body_hair::BodyHair, body_hair_color_b::BodyHairColorB,
                body_hair_color_g::BodyHairColorG, body_hair_color_r::BodyHairColorR,
                body_scale_abdomen::BodyScaleAbdomen, body_scale_arm_left::BodyScaleArmLeft,
                body_scale_arm_right::BodyScaleArmRight, body_scale_breast::BodyScaleBreast,
                body_scale_head::BodyScaleHead, body_scale_leg_left::BodyScaleLegLeft,
                body_scale_leg_right::BodyScaleLegRight, brow_ridge_height::BrowRidgeHeight,
                cheekbone_depth::CheekboneDepth, cheekbone_height::CheekboneHeight,
                cheekbone_protrusion::CheekboneProtrusion, cheekbone_width::CheekboneWidth,
                cheeks::Cheeks, cheeks_color_b::CheeksColorB, cheeks_color_g::CheeksColorG,
                cheeks_color_r::CheeksColorR, cheeks_morph::CheeksMorph, chin_depth::ChinDepth,
                chin_height::ChinHeight, chin_length::ChinLength, chin_protrusion::ChinProtrusion,
                chin_size::ChinSize, chin_tip_position::ChinTipPosition, chin_width::ChinWidth,
                decal_angle::DecalAngle, decal_color_b::DecalColorB, decal_color_g::DecalColorG,
                decal_color_r::DecalColorR, decal_flip::DecalFlip,
                decal_position_x::DecalPositionX, decal_position_y::DecalPositionY,
                decal_scale::DecalScale, eye_left_clouding::EyeLeftClouding,
                eye_left_clouding_b::EyeLeftCloudingB, eye_left_clouding_g::EyeLeftCloudingG,
                eye_left_clouding_r::EyeLeftCloudingR, eye_left_color_b::EyeLeftColorB,
                eye_left_color_g::EyeLeftColorG, eye_left_color_r::EyeLeftColorR,
                eye_left_iris_size::EyeLeftIrisSize, eye_left_position::EyeLeftPosition,
                eye_left_sclera_b::EyeLeftScleraB, eye_left_sclera_g::EyeLeftScleraG,
                eye_left_sclera_r::EyeLeftScleraR, eye_position::EyePosition,
                eye_right_clouding::EyeRightClouding, eye_right_clouding_b::EyeRightCloudingB,
                eye_right_clouding_g::EyeRightCloudingG, eye_right_clouding_r::EyeRightCloudingR,
                eye_right_color_b::EyeRightColorB, eye_right_color_g::EyeRightColorG,
                eye_right_color_r::EyeRightColorR, eye_right_iris_size::EyeRightIrisSize,
                eye_right_position::EyeRightPosition, eye_right_sclera_b::EyeRightScleraB,
                eye_right_sclera_g::EyeRightScleraG, eye_right_sclera_r::EyeRightScleraR,
                eye_size::EyeSize, eye_slant::EyeSlant, eye_spacing::EyeSpacing,
                eyebrow_color_a::EyebrowColorA, eyebrow_color_b::EyebrowColorB,
                eyebrow_color_g::EyebrowColorG, eyebrow_color_r::EyebrowColorR,
                eyebrow_color_root_darkness::EyebrowColorRootDarkness,
                eyebrow_color_white::EyebrowColorWhite, eyelash_color_b::EyelashColorB,
                eyelash_color_g::EyelashColorG, eyelash_color_r::EyelashColorR, eyeliner::Eyeliner,
                eyeliner_color_b::EyelinerColorB, eyeliner_color_g::EyelinerColorG,
                eyeliner_color_r::EyelinerColorR, eyeshadow_lower::EyeshadowLower,
                eyeshadow_lower_color_b::EyeshadowLowerColorB,
                eyeshadow_lower_color_g::EyeshadowLowerColorG,
                eyeshadow_lower_color_r::EyeshadowLowerColorR, eyeshadow_upper::EyeshadowUpper,
                eyeshadow_upper_color_b::EyeshadowUpperColorB,
                eyeshadow_upper_color_g::EyeshadowUpperColorG,
                eyeshadow_upper_color_r::EyeshadowUpperColorR, face_protrusion::FaceProtrusion,
                facial_aesthetic::FacialAesthetic, facial_feature_slant::FacialFeatureSlant,
                forehead_depth::ForeheadDepth, forehead_protrusion::ForeheadProtrusion,
                form_emphasis::FormEmphasis, frenzied_flame::FrenziedFlame,
                hair_color_a::HairColorA, hair_color_b::HairColorB, hair_color_g::HairColorG,
                hair_color_r::HairColorR, hair_color_root_darkness::HairColorRootDarkness,
                hair_color_white::HairColorWhite, horizontal_face_ratio::HorizontalFaceRatio,
                inner_brow_height::InnerBrowHeight, jaw_contour::JawContour,
                jaw_protrusion::JawProtrusion, jaw_width::JawWidth, lip_fullness::LipFullness,
                lip_protrusion::LipProtrusion, lip_shape::LipShape, lip_size::LipSize,
                lip_thickness::LipThickness, lipstick::Lipstick, lipstick_color_b::LipstickColorB,
                lipstick_color_g::LipstickColorG, lipstick_color_r::LipstickColorR,
                lower_jaw::LowerJaw, mouth_expression::MouthExpression,
                mouth_occlusion::MouthOcclusion, mouth_position::MouthPosition,
                mouth_protrusion::MouthProtrusion, mouth_slant::MouthSlant,
                mouth_to_chin_distance::MouthToChinDistance, mouth_width::MouthWidth,
                nose_bridge_height::NoseBridgeHeight,
                nose_bridge_protrusion1::NoseBridgeProtrusion1,
                nose_bridge_protrusion2::NoseBridgeProtrusion2, nose_bridge_width::NoseBridgeWidth,
                nose_height::NoseHeight, nose_position::NosePosition,
                nose_protrusion::NoseProtrusion, nose_ridge_depth::NoseRidgeDepth,
                nose_ridge_length::NoseRidgeLength, nose_size::NoseSize, nose_slant::NoseSlant,
                nose_tip_height::NoseTipHeight, nose_to_forehead_ratio::NoseToForeheadRatio,
                nostril_size::NostrilSize, nostril_slant::NostrilSlant,
                nostril_width::NostrilWidth, numen::Numen, outer_brow_ridge::OuterBrowRidge,
                skin_color_a::SkinColorA, skin_color_b::SkinColorB, skin_color_g::SkinColorG,
                skin_color_r::SkinColorR, skin_dark_circle::SkinDarkCircle,
                skin_dark_circle_color_b::SkinDarkCircleColorB,
                skin_dark_circle_color_g::SkinDarkCircleColorG,
                skin_dark_circle_color_r::SkinDarkCircleColorR, skin_pores::SkinPores,
                unimplemented1::Unimplemented1, unimplemented10::Unimplemented10,
                unimplemented11::Unimplemented11, unimplemented12::Unimplemented12,
                unimplemented2::Unimplemented2, unimplemented3::Unimplemented3,
                unimplemented4::Unimplemented4, unimplemented5::Unimplemented5,
                unimplemented6::Unimplemented6, unimplemented7::Unimplemented7,
                unimplemented8::Unimplemented8, unimplemented9::Unimplemented9,
                vert_face_ratio::VertFaceRatio,
            },
            magic_bytes::MagicBytes,
        },
        traits::binary_readable::BinaryReadable,
    };

    #[test]
    fn _test_read_arbitrary_bytes() {
        let mut dynamic_bytes = vec![0x00; 0x100];
        // let mut dynamic_bytes = vec![0x00; 0x6010];
        let file =
            std::fs::File::open("testdata/vagabond/save_slots/0/gesture_game_data.sl2").unwrap();
        // let file = std::fs::File::open("testdata/vagabond/save_slots/0/inventory_storage_box.sl2").unwrap();
        let mut reader = BufReader::new(file);
        std::io::Read::read_exact(&mut reader, &mut dynamic_bytes).unwrap();
        dynamic_bytes.iter().for_each(|byte| {
            print!("{:02X}\u{2008}", byte);
        });
    }

    #[test]
    fn test_read_save_face_data() {
        let mut expected_face_data: Vec<u8> = Vec::new();
        let initial_data: [u8; 16] = MagicBytes::read(
            &mut (BufReader::new(Cursor::new([
                0xFF, 0xFF, 0xFF, 0xFF, 0x46, 0x41, 0x43, 0x45, 0x04, 0x00, 0x00, 0x00, 0x20, 0x01,
                0x00, 0x00,
            ]))),
        )
        .unwrap()
        .data
        .data
        .try_into()
        .expect("Could not convert to [u8; 16] from Vec<u8>");

        use std::convert::TryInto;

        let face_model: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
        let hair_model: [u8; 4] = [0x09, 0x00, 0x00, 0x00];
        let eye_model: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
        let eyebrow_model: [u8; 4] = [0x03, 0x00, 0x00, 0x00];
        let beard_model: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
        let acc_model: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
        let decal_model: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
        let eyelash_model: [u8; 4] = [0x02, 0x00, 0x00, 0x00];

        let apparent_age: [u8; 1] = ApparentAge::read(&mut (BufReader::new(Cursor::new([0xCD]))))
            .unwrap()
            .data
            .into();
        let facial_aesthetic: [u8; 1] =
            FacialAesthetic::read(&mut (BufReader::new(Cursor::new([0x6C]))))
                .unwrap()
                .data
                .into();
        let form_emphasis: [u8; 1] = FormEmphasis::read(&mut (BufReader::new(Cursor::new([0x00]))))
            .unwrap()
            .data
            .into();
        let numen: [u8; 1] = Numen::read(&mut (BufReader::new(Cursor::new([0x00]))))
            .unwrap()
            .data
            .into();
        let brow_ridge_height: [u8; 1] =
            BrowRidgeHeight::read(&mut (BufReader::new(Cursor::new([0x76]))))
                .unwrap()
                .data
                .into();
        let inner_brow_height: [u8; 1] =
            InnerBrowHeight::read(&mut (BufReader::new(Cursor::new([0xA8]))))
                .unwrap()
                .data
                .into();
        let outer_brow_ridge: [u8; 1] =
            OuterBrowRidge::read(&mut (BufReader::new(Cursor::new([0x6C]))))
                .unwrap()
                .data
                .into();
        let cheekbone_height: [u8; 1] =
            CheekboneHeight::read(&mut (BufReader::new(Cursor::new([0x58]))))
                .unwrap()
                .data
                .into();
        let cheekbone_depth: [u8; 1] =
            CheekboneDepth::read(&mut (BufReader::new(Cursor::new([0xBC]))))
                .unwrap()
                .data
                .into();
        let cheekbone_width: [u8; 1] =
            CheekboneWidth::read(&mut (BufReader::new(Cursor::new([0x62]))))
                .unwrap()
                .data
                .into();
        let cheekbone_protrusion: [u8; 1] =
            CheekboneProtrusion::read(&mut (BufReader::new(Cursor::new([0x94]))))
                .unwrap()
                .data
                .into();
        let cheeks_morph: [u8; 1] = CheeksMorph::read(&mut (BufReader::new(Cursor::new([0x4E]))))
            .unwrap()
            .data
            .into();
        let chin_tip_position: [u8; 1] =
            ChinTipPosition::read(&mut (BufReader::new(Cursor::new([0xA8]))))
                .unwrap()
                .data
                .into();
        let chin_length: [u8; 1] = ChinLength::read(&mut (BufReader::new(Cursor::new([0x62]))))
            .unwrap()
            .data
            .into();
        let chin_protrusion: [u8; 1] =
            ChinProtrusion::read(&mut (BufReader::new(Cursor::new([0x44]))))
                .unwrap()
                .data
                .into();
        let chin_depth: [u8; 1] = ChinDepth::read(&mut (BufReader::new(Cursor::new([0x80]))))
            .unwrap()
            .data
            .into();
        let chin_size: [u8; 1] = ChinSize::read(&mut (BufReader::new(Cursor::new([0x8A]))))
            .unwrap()
            .data
            .into();
        let chin_height: [u8; 1] = ChinHeight::read(&mut (BufReader::new(Cursor::new([0x6C]))))
            .unwrap()
            .data
            .into();
        let chin_width: [u8; 1] = ChinWidth::read(&mut (BufReader::new(Cursor::new([0x6C]))))
            .unwrap()
            .data
            .into();
        let eye_position: [u8; 1] = EyePosition::read(&mut (BufReader::new(Cursor::new([0x8A]))))
            .unwrap()
            .data
            .into();
        let eye_size: [u8; 1] = EyeSize::read(&mut (BufReader::new(Cursor::new([0x8A]))))
            .unwrap()
            .data
            .into();
        let eye_slant: [u8; 1] = EyeSlant::read(&mut (BufReader::new(Cursor::new([0xC6]))))
            .unwrap()
            .data
            .into();
        let eye_spacing: [u8; 1] = EyeSpacing::read(&mut (BufReader::new(Cursor::new([0x3A]))))
            .unwrap()
            .data
            .into();
        let nose_size: [u8; 1] = NoseSize::read(&mut (BufReader::new(Cursor::new([0x80]))))
            .unwrap()
            .data
            .into();
        let nose_to_forehead_ratio: [u8; 1] =
            NoseToForeheadRatio::read(&mut (BufReader::new(Cursor::new([0xB2]))))
                .unwrap()
                .data
                .into();

        let unimplemented_1: [u8; 1] =
            Unimplemented1::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let face_protrusion: [u8; 1] =
            FaceProtrusion::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let vert_face_ratio: [u8; 1] =
            VertFaceRatio::read(&mut (BufReader::new(Cursor::new([0x4E]))))
                .unwrap()
                .data
                .into();
        let facial_feature_slant: [u8; 1] =
            FacialFeatureSlant::read(&mut (BufReader::new(Cursor::new([0x44]))))
                .unwrap()
                .data
                .into();
        let horizontal_face_ratio: [u8; 1] =
            HorizontalFaceRatio::read(&mut (BufReader::new(Cursor::new([0x73]))))
                .unwrap()
                .data
                .into();
        let unimplemented_2: [u8; 1] =
            Unimplemented2::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let forehead_depth: [u8; 1] =
            ForeheadDepth::read(&mut (BufReader::new(Cursor::new([0xB2]))))
                .unwrap()
                .data
                .into();
        let forehead_protrusion: [u8; 1] =
            ForeheadProtrusion::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let unimplemented_3: [u8; 1] =
            Unimplemented3::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let jaw_protrusion: [u8; 1] =
            JawProtrusion::read(&mut (BufReader::new(Cursor::new([0x76]))))
                .unwrap()
                .data
                .into();
        let jaw_width: [u8; 1] = JawWidth::read(&mut (BufReader::new(Cursor::new([0x6C]))))
            .unwrap()
            .data
            .into();
        let lower_jaw: [u8; 1] = LowerJaw::read(&mut (BufReader::new(Cursor::new([0x62]))))
            .unwrap()
            .data
            .into();
        let jaw_contour: [u8; 1] = JawContour::read(&mut (BufReader::new(Cursor::new([0x76]))))
            .unwrap()
            .data
            .into();
        let lip_shape: [u8; 1] = LipShape::read(&mut (BufReader::new(Cursor::new([0x80]))))
            .unwrap()
            .data
            .into();
        let lip_size: [u8; 1] = LipSize::read(&mut (BufReader::new(Cursor::new([0xA8]))))
            .unwrap()
            .data
            .into();
        let lip_fullness: [u8; 1] = LipFullness::read(&mut (BufReader::new(Cursor::new([0x62]))))
            .unwrap()
            .data
            .into();
        let mouth_expression: [u8; 1] =
            MouthExpression::read(&mut (BufReader::new(Cursor::new([0x8C]))))
                .unwrap()
                .data
                .into();
        let lip_protrusion: [u8; 1] =
            LipProtrusion::read(&mut (BufReader::new(Cursor::new([0xC6]))))
                .unwrap()
                .data
                .into();
        let lip_thickness: [u8; 1] = LipThickness::read(&mut (BufReader::new(Cursor::new([0x8A]))))
            .unwrap()
            .data
            .into();
        let mouth_protrusion: [u8; 1] =
            MouthProtrusion::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let mouth_slant: [u8; 1] = MouthSlant::read(&mut (BufReader::new(Cursor::new([0xA8]))))
            .unwrap()
            .data
            .into();
        let mouth_occlusion: [u8; 1] =
            MouthOcclusion::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let mouth_position: [u8; 1] =
            MouthPosition::read(&mut (BufReader::new(Cursor::new([0x76]))))
                .unwrap()
                .data
                .into();
        let mouth_width: [u8; 1] = MouthWidth::read(&mut (BufReader::new(Cursor::new([0x69]))))
            .unwrap()
            .data
            .into();
        let mouth_to_chin_distance: [u8; 1] =
            MouthToChinDistance::read(&mut (BufReader::new(Cursor::new([0xD0]))))
                .unwrap()
                .data
                .into();
        let nose_ridge_depth: [u8; 1] =
            NoseRidgeDepth::read(&mut (BufReader::new(Cursor::new([0x8A]))))
                .unwrap()
                .data
                .into();
        let nose_ridge_length: [u8; 1] =
            NoseRidgeLength::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let nose_position: [u8; 1] = NosePosition::read(&mut (BufReader::new(Cursor::new([0x80]))))
            .unwrap()
            .data
            .into();
        let nose_tip_height: [u8; 1] =
            NoseTipHeight::read(&mut (BufReader::new(Cursor::new([0x87]))))
                .unwrap()
                .data
                .into();
        let nostril_slant: [u8; 1] = NostrilSlant::read(&mut (BufReader::new(Cursor::new([0xC6]))))
            .unwrap()
            .data
            .into();
        let nostril_size: [u8; 1] = NostrilSize::read(&mut (BufReader::new(Cursor::new([0x62]))))
            .unwrap()
            .data
            .into();
        let nostril_width: [u8; 1] = NostrilWidth::read(&mut (BufReader::new(Cursor::new([0xC6]))))
            .unwrap()
            .data
            .into();
        let nose_protrusion: [u8; 1] =
            NoseProtrusion::read(&mut (BufReader::new(Cursor::new([0x7D]))))
                .unwrap()
                .data
                .into();
        let nose_bridge_height: [u8; 1] =
            NoseBridgeHeight::read(&mut (BufReader::new(Cursor::new([0x26]))))
                .unwrap()
                .data
                .into();
        let nose_bridge_protrusion_1: [u8; 1] =
            NoseBridgeProtrusion1::read(&mut (BufReader::new(Cursor::new([0xBC]))))
                .unwrap()
                .data
                .into();
        let nose_bridge_protrusion_2: [u8; 1] =
            NoseBridgeProtrusion2::read(&mut (BufReader::new(Cursor::new([0x55]))))
                .unwrap()
                .data
                .into();
        let nose_bridge_width: [u8; 1] =
            NoseBridgeWidth::read(&mut (BufReader::new(Cursor::new([0x8A]))))
                .unwrap()
                .data
                .into();
        let nose_height: [u8; 1] = NoseHeight::read(&mut (BufReader::new(Cursor::new([0x9E]))))
            .unwrap()
            .data
            .into();
        let nose_slant: [u8; 1] = NoseSlant::read(&mut (BufReader::new(Cursor::new([0x46]))))
            .unwrap()
            .data
            .into();

        let unimplemented_4: [u8; 64] = Unimplemented4::read(
            &mut (BufReader::new(Cursor::new([
                0x80, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x80,
                0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
                0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80,
                0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x00,
            ]))),
        )
        .unwrap()
        .attributes
        .data
        .try_into()
        .expect("Could not convert to [u8; 16] from Vec<u8>");

        let body_scale_head: [u8; 1] =
            BodyScaleHead::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let body_scale_breast: [u8; 1] =
            BodyScaleBreast::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let body_scale_abdomen: [u8; 1] =
            BodyScaleAbdomen::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let body_scale_arm_right: [u8; 1] =
            BodyScaleArmRight::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let body_scale_leg_right: [u8; 1] =
            BodyScaleLegRight::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let body_scale_arm_left: [u8; 1] =
            BodyScaleArmLeft::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let body_scale_leg_left: [u8; 1] =
            BodyScaleLegLeft::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let skin_color_r: [u8; 1] = SkinColorR::read(&mut (BufReader::new(Cursor::new([0x8F]))))
            .unwrap()
            .data
            .into();
        let skin_color_g: [u8; 1] = SkinColorG::read(&mut (BufReader::new(Cursor::new([0x67]))))
            .unwrap()
            .data
            .into();
        let skin_color_b: [u8; 1] = SkinColorB::read(&mut (BufReader::new(Cursor::new([0x4F]))))
            .unwrap()
            .data
            .into();
        let skin_color_a: [u8; 1] = SkinColorA::read(&mut (BufReader::new(Cursor::new([0xA0]))))
            .unwrap()
            .data
            .into();
        let skin_pores: [u8; 1] = SkinPores::read(&mut (BufReader::new(Cursor::new([0xFF]))))
            .unwrap()
            .data
            .into();
        let beard_stubble: [u8; 1] = BeardStubble::read(&mut (BufReader::new(Cursor::new([0xD7]))))
            .unwrap()
            .data
            .into();
        let skin_dark_circle: [u8; 1] =
            SkinDarkCircle::read(&mut (BufReader::new(Cursor::new([0x50]))))
                .unwrap()
                .data
                .into();
        let skin_dark_circle_color_r: [u8; 1] =
            SkinDarkCircleColorR::read(&mut (BufReader::new(Cursor::new([0x14]))))
                .unwrap()
                .data
                .into();
        let skin_dark_circle_color_g: [u8; 1] =
            SkinDarkCircleColorG::read(&mut (BufReader::new(Cursor::new([0x1E]))))
                .unwrap()
                .data
                .into();
        let skin_dark_circle_color_b: [u8; 1] =
            SkinDarkCircleColorB::read(&mut (BufReader::new(Cursor::new([0x19]))))
                .unwrap()
                .data
                .into();
        let cheeks: [u8; 1] = Cheeks::read(&mut (BufReader::new(Cursor::new([0x00]))))
            .unwrap()
            .data
            .into();
        let cheeks_color_r: [u8; 1] =
            CheeksColorR::read(&mut (BufReader::new(Cursor::new([0xFF]))))
                .unwrap()
                .data
                .into();
        let cheeks_color_g: [u8; 1] =
            CheeksColorG::read(&mut (BufReader::new(Cursor::new([0x41]))))
                .unwrap()
                .data
                .into();
        let cheeks_color_b: [u8; 1] =
            CheeksColorB::read(&mut (BufReader::new(Cursor::new([0x41]))))
                .unwrap()
                .data
                .into();
        let eyeliner: [u8; 1] = Eyeliner::read(&mut (BufReader::new(Cursor::new([0x00]))))
            .unwrap()
            .data
            .into();
        let eyeliner_color_r: [u8; 1] =
            EyelinerColorR::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let eyeliner_color_g: [u8; 1] =
            EyelinerColorG::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let eyeliner_color_b: [u8; 1] =
            EyelinerColorB::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let eyeshadow_lower: [u8; 1] =
            EyeshadowLower::read(&mut (BufReader::new(Cursor::new([0x64]))))
                .unwrap()
                .data
                .into();
        let eyeshadow_lower_color_r: [u8; 1] =
            EyeshadowLowerColorR::read(&mut (BufReader::new(Cursor::new([0x32]))))
                .unwrap()
                .data
                .into();
        let eyeshadow_lower_color_g: [u8; 1] =
            EyeshadowLowerColorG::read(&mut (BufReader::new(Cursor::new([0x19]))))
                .unwrap()
                .data
                .into();
        let eyeshadow_lower_color_b: [u8; 1] =
            EyeshadowLowerColorB::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let eyeshadow_upper: [u8; 1] =
            EyeshadowUpper::read(&mut (BufReader::new(Cursor::new([0x1E]))))
                .unwrap()
                .data
                .into();
        let eyeshadow_upper_color_r: [u8; 1] =
            EyeshadowUpperColorR::read(&mut (BufReader::new(Cursor::new([0x28]))))
                .unwrap()
                .data
                .into();
        let eyeshadow_upper_color_g: [u8; 1] =
            EyeshadowUpperColorG::read(&mut (BufReader::new(Cursor::new([0x14]))))
                .unwrap()
                .data
                .into();
        let eyeshadow_upper_color_b: [u8; 1] =
            EyeshadowUpperColorB::read(&mut (BufReader::new(Cursor::new([0x1E]))))
                .unwrap()
                .data
                .into();
        let lipstick: [u8; 1] = Lipstick::read(&mut (BufReader::new(Cursor::new([0x14]))))
            .unwrap()
            .data
            .into();
        let lipstick_color_r: [u8; 1] =
            LipstickColorR::read(&mut (BufReader::new(Cursor::new([0xFF]))))
                .unwrap()
                .data
                .into();
        let lipstick_color_g: [u8; 1] =
            LipstickColorG::read(&mut (BufReader::new(Cursor::new([0x57]))))
                .unwrap()
                .data
                .into();
        let lipstick_color_b: [u8; 1] =
            LipstickColorB::read(&mut (BufReader::new(Cursor::new([0x57]))))
                .unwrap()
                .data
                .into();
        let decal_position_x: [u8; 1] =
            DecalPositionX::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let decal_position_y: [u8; 1] =
            DecalPositionY::read(&mut (BufReader::new(Cursor::new([0xD2]))))
                .unwrap()
                .data
                .into();
        let decal_angle: [u8; 1] = DecalAngle::read(&mut (BufReader::new(Cursor::new([0x80]))))
            .unwrap()
            .data
            .into();
        let decal_scale: [u8; 1] = DecalScale::read(&mut (BufReader::new(Cursor::new([0x80]))))
            .unwrap()
            .data
            .into();
        let decal_color_r: [u8; 1] = DecalColorR::read(&mut (BufReader::new(Cursor::new([0x47]))))
            .unwrap()
            .data
            .into();
        let decal_color_g: [u8; 1] = DecalColorG::read(&mut (BufReader::new(Cursor::new([0x25]))))
            .unwrap()
            .data
            .into();
        let decal_color_b: [u8; 1] = DecalColorB::read(&mut (BufReader::new(Cursor::new([0x18]))))
            .unwrap()
            .data
            .into();
        let unimplemented_5: [u8; 1] =
            Unimplemented5::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let decal_flip: [u8; 1] = DecalFlip::read(&mut (BufReader::new(Cursor::new([0x01]))))
            .unwrap()
            .data
            .into();
        let body_hair: [u8; 1] = BodyHair::read(&mut (BufReader::new(Cursor::new([0x00]))))
            .unwrap()
            .data
            .into();
        let body_hair_color_r: [u8; 1] =
            BodyHairColorR::read(&mut (BufReader::new(Cursor::new([0x46]))))
                .unwrap()
                .data
                .into();
        let body_hair_color_g: [u8; 1] =
            BodyHairColorG::read(&mut (BufReader::new(Cursor::new([0x30]))))
                .unwrap()
                .data
                .into();
        let body_hair_color_b: [u8; 1] =
            BodyHairColorB::read(&mut (BufReader::new(Cursor::new([0x1D]))))
                .unwrap()
                .data
                .into();
        let eye_right_color_r: [u8; 1] =
            EyeRightColorR::read(&mut (BufReader::new(Cursor::new([0x1A]))))
                .unwrap()
                .data
                .into();
        let eye_right_color_g: [u8; 1] =
            EyeRightColorG::read(&mut (BufReader::new(Cursor::new([0x0F]))))
                .unwrap()
                .data
                .into();
        let eye_right_color_b: [u8; 1] =
            EyeRightColorB::read(&mut (BufReader::new(Cursor::new([0x05]))))
                .unwrap()
                .data
                .into();
        let eye_right_iris_size: [u8; 1] =
            EyeRightIrisSize::read(&mut (BufReader::new(Cursor::new([0xC8]))))
                .unwrap()
                .data
                .into();
        let eye_right_clouding: [u8; 1] =
            EyeRightClouding::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let eye_right_clouding_r: [u8; 1] =
            EyeRightCloudingR::read(&mut (BufReader::new(Cursor::new([0x64]))))
                .unwrap()
                .data
                .into();
        let eye_right_clouding_g: [u8; 1] =
            EyeRightCloudingG::read(&mut (BufReader::new(Cursor::new([0x64]))))
                .unwrap()
                .data
                .into();
        let eye_right_clouding_b: [u8; 1] =
            EyeRightCloudingB::read(&mut (BufReader::new(Cursor::new([0x64]))))
                .unwrap()
                .data
                .into();
        let eye_right_sclera_r: [u8; 1] =
            EyeRightScleraR::read(&mut (BufReader::new(Cursor::new([0xFF]))))
                .unwrap()
                .data
                .into();
        let eye_right_sclera_g: [u8; 1] =
            EyeRightScleraG::read(&mut (BufReader::new(Cursor::new([0xFF]))))
                .unwrap()
                .data
                .into();
        let eye_right_sclera_b: [u8; 1] =
            EyeRightScleraB::read(&mut (BufReader::new(Cursor::new([0xFF]))))
                .unwrap()
                .data
                .into();
        let eye_right_position: [u8; 1] =
            EyeRightPosition::read(&mut (BufReader::new(Cursor::new([0x8A]))))
                .unwrap()
                .data
                .into();
        let eye_left_color_r: [u8; 1] =
            EyeLeftColorR::read(&mut (BufReader::new(Cursor::new([0x1A]))))
                .unwrap()
                .data
                .into();
        let eye_left_color_g: [u8; 1] =
            EyeLeftColorG::read(&mut (BufReader::new(Cursor::new([0x0F]))))
                .unwrap()
                .data
                .into();
        let eye_left_color_b: [u8; 1] =
            EyeLeftColorB::read(&mut (BufReader::new(Cursor::new([0x05]))))
                .unwrap()
                .data
                .into();
        let eye_left_iris_size: [u8; 1] =
            EyeLeftIrisSize::read(&mut (BufReader::new(Cursor::new([0xC8]))))
                .unwrap()
                .data
                .into();
        let eye_left_clouding: [u8; 1] =
            EyeLeftClouding::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let eye_left_clouding_r: [u8; 1] =
            EyeLeftCloudingR::read(&mut (BufReader::new(Cursor::new([0x64]))))
                .unwrap()
                .data
                .into();
        let eye_left_clouding_g: [u8; 1] =
            EyeLeftCloudingG::read(&mut (BufReader::new(Cursor::new([0x64]))))
                .unwrap()
                .data
                .into();
        let eye_left_clouding_b: [u8; 1] =
            EyeLeftCloudingB::read(&mut (BufReader::new(Cursor::new([0x64]))))
                .unwrap()
                .data
                .into();
        let eye_left_sclera_r: [u8; 1] =
            EyeLeftScleraR::read(&mut (BufReader::new(Cursor::new([0xFF]))))
                .unwrap()
                .data
                .into();
        let eye_left_sclera_g: [u8; 1] =
            EyeLeftScleraG::read(&mut (BufReader::new(Cursor::new([0xFF]))))
                .unwrap()
                .data
                .into();
        let eye_left_sclera_b: [u8; 1] =
            EyeLeftScleraB::read(&mut (BufReader::new(Cursor::new([0xFF]))))
                .unwrap()
                .data
                .into();
        let eye_left_position: [u8; 1] =
            EyeLeftPosition::read(&mut (BufReader::new(Cursor::new([0x8A]))))
                .unwrap()
                .data
                .into();
        let hair_color_r: [u8; 1] = HairColorR::read(&mut (BufReader::new(Cursor::new([0x46]))))
            .unwrap()
            .data
            .into();
        let hair_color_g: [u8; 1] = HairColorG::read(&mut (BufReader::new(Cursor::new([0x30]))))
            .unwrap()
            .data
            .into();
        let hair_color_b: [u8; 1] = HairColorB::read(&mut (BufReader::new(Cursor::new([0x1D]))))
            .unwrap()
            .data
            .into();
        let hair_color_a: [u8; 1] = HairColorA::read(&mut (BufReader::new(Cursor::new([0x4E]))))
            .unwrap()
            .data
            .into();
        let hair_color_root_darkness: [u8; 1] =
            HairColorRootDarkness::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let hair_color_white: [u8; 1] =
            HairColorWhite::read(&mut (BufReader::new(Cursor::new([0x50]))))
                .unwrap()
                .data
                .into();
        let beard_color_r: [u8; 1] = BeardColorR::read(&mut (BufReader::new(Cursor::new([0x46]))))
            .unwrap()
            .data
            .into();
        let beard_color_g: [u8; 1] = BeardColorG::read(&mut (BufReader::new(Cursor::new([0x30]))))
            .unwrap()
            .data
            .into();
        let beard_color_b: [u8; 1] = BeardColorB::read(&mut (BufReader::new(Cursor::new([0x1D]))))
            .unwrap()
            .data
            .into();
        let beard_color_a: [u8; 1] = BeardColorA::read(&mut (BufReader::new(Cursor::new([0x4E]))))
            .unwrap()
            .data
            .into();
        let beard_color_root_darkness: [u8; 1] =
            BeardColorRootDarkness::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let beard_color_white: [u8; 1] =
            BeardColorWhite::read(&mut (BufReader::new(Cursor::new([0x50]))))
                .unwrap()
                .data
                .into();
        let eyebrow_color_r: [u8; 1] =
            EyebrowColorR::read(&mut (BufReader::new(Cursor::new([0x46]))))
                .unwrap()
                .data
                .into();
        let eyebrow_color_g: [u8; 1] =
            EyebrowColorG::read(&mut (BufReader::new(Cursor::new([0x30]))))
                .unwrap()
                .data
                .into();
        let eyebrow_color_b: [u8; 1] =
            EyebrowColorB::read(&mut (BufReader::new(Cursor::new([0x1D]))))
                .unwrap()
                .data
                .into();
        let eyebrow_color_a: [u8; 1] =
            EyebrowColorA::read(&mut (BufReader::new(Cursor::new([0x4E]))))
                .unwrap()
                .data
                .into();
        let eyebrow_color_root_darkness: [u8; 1] =
            EyebrowColorRootDarkness::read(&mut (BufReader::new(Cursor::new([0x80]))))
                .unwrap()
                .data
                .into();
        let eyebrow_color_white: [u8; 1] =
            EyebrowColorWhite::read(&mut (BufReader::new(Cursor::new([0x50]))))
                .unwrap()
                .data
                .into();
        let eyelash_color_r: [u8; 1] =
            EyelashColorR::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let eyelash_color_g: [u8; 1] =
            EyelashColorG::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let eyelash_color_b: [u8; 1] =
            EyelashColorB::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let accessories_color_r: [u8; 1] =
            AccessoriesColorR::read(&mut (BufReader::new(Cursor::new([0x3C]))))
                .unwrap()
                .data
                .into();
        let accessories_color_g: [u8; 1] =
            AccessoriesColorG::read(&mut (BufReader::new(Cursor::new([0x3C]))))
                .unwrap()
                .data
                .into();
        let accessories_color_b: [u8; 1] =
            AccessoriesColorB::read(&mut (BufReader::new(Cursor::new([0x3C]))))
                .unwrap()
                .data
                .into();
        let frenzied_flame: [u8; 1] =
            FrenziedFlame::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let unimplemented_6: [u8; 1] =
            Unimplemented6::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let unimplemented_7: [u8; 1] =
            Unimplemented7::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let unimplemented_8: [u8; 1] =
            Unimplemented8::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let unimplemented_9: [u8; 1] =
            Unimplemented9::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let unimplemented_10: [u8; 1] =
            Unimplemented10::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let unimplemented_11: [u8; 1] =
            Unimplemented11::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();
        let unimplemented_12: [u8; 1] =
            Unimplemented12::read(&mut (BufReader::new(Cursor::new([0x00]))))
                .unwrap()
                .data
                .into();

        let end_data: [u8; 21] = [
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
        expected_face_data.extend_from_slice(&nose_bridge_width);
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

        assert_eq!(unk01.data.data, 0x00000097)
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
