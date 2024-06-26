#[macro_export]
macro_rules! impl_byte_array_readable {
    ($name:ident,$size:expr) => {
        use std::{
            fmt,
            ops::{Deref, DerefMut},
        };

        use crate::traits::validate::Validate;
        use crate::traits::byte_array_readable::ByteArrayReadable;
        
        type Reader<const N: usize> = crate::models::shared::byte_array_reader::ByteArray<N>;

        #[derive(serde::Deserialize, serde::Serialize, Clone)]
        pub struct $name {
            pub data: Reader<$size>,
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    data: Reader::<$size>::default(),
                }
            }
        }

        // Implement Deref trait for $name
        impl Deref for $name {
            type Target = Reader<$size>;

            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }

        // Implement DerefMut trait for $name
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.data
            }
        }

        // Implement Debug trait for $name by forwarding to Reader
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(&self.data, f)
            }
        }

        // Implement BinaryReadable trait for $name by forwarding to Reader
        impl ByteArrayReadable for $name {
            fn read<R: std::io::Read + std::io::Seek>(reader: &mut R) -> Result<Self, std::io::Error> {
                Ok($name {
                    data: Reader::<$size>::read(reader)?,
                })
            }
        }

        impl Validate for $name {
            fn validate(&self) -> bool {
                self.data.validate()
            }
        }
    };
}
