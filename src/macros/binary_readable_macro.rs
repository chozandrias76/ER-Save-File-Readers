#[macro_export]
macro_rules! impl_binary_readable {
    ($name:ident, $attribute_name:ident, $attribute_impl:ty) => {
        use crate::BinaryReadable;
        use crate::traits::byte_array_readable::ByteArrayReadable;

        impl BinaryReadable for $name {
            fn read<R: std::io::Read + std::io::Seek>(
                reader: &mut R,
            ) -> Result<Self, std::io::Error> {
                match <$attribute_impl>::read(reader) {
                    Ok($attribute_name) => Ok($name { $attribute_name }),
                    Err(e) => Err(e),
                }
            }
        }
    };
}
