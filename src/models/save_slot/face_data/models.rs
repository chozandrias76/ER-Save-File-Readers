pub mod accessory;
pub mod beard;
pub mod decal;
pub mod eye;
pub mod eyebrow;
pub mod eyelash;
pub mod face;
pub mod hair;
use crate::impl_byte_array_readable;

impl_byte_array_readable!(Model, 4);