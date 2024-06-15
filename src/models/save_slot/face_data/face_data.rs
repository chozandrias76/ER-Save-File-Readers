use crate::impl_byte_array_readable;

use super::{
    attributes::apparent_age::ApparentAge,
    magic_bytes::MagicBytes,
    models::{
        accessory::Accessory, beard::Beard, decal::Decal, eye::Eye, eyebrow::Eyebrow,
        eyelash::Eyelash, face::Face, hair::Hair,
    },
};

impl_byte_array_readable!(FaceData, 303);

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct FaceDatum {
    pub datum: (
        MagicBytes,
        Face,
        Hair,
        Eye,
        Eyebrow,
        Beard,
        Accessory,
        Decal,
        Eyelash,
        ApparentAge,
    ),
}