#[macro_export]
macro_rules! impl_attribute {
    ($name:ident) => {
        use crate::impl_u8_readable;
        impl_u8_readable!($name);
    };
}
