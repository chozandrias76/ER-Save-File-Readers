#[macro_export]
macro_rules! impl_u32_readable {
    ($name:ident) => {
        use std::{
            fmt,
            io::{self, Read, Seek},
            ops::{Deref, DerefMut},
        };

        use crate::{
            models::shared::u32_reader::U32Reader,
            traits::binary_readable::BinaryReadable,
        };
        use crate::traits::validate::Validate;

        #[derive(serde::Deserialize, serde::Serialize, Clone)]
        pub struct $name {
            pub inner: U32Reader,
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    inner: U32Reader::default(),
                }
            }
        }

        // Implement Deref trait for $name
        impl Deref for $name {
            type Target = U32Reader;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        // Implement DerefMut trait for $name
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        // Implement Debug trait for $name by forwarding to U32Reader
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(&self.inner, f)
            }
        }

        // Implement BinaryReadable trait for $name by forwarding to U32Reader
        impl BinaryReadable for $name {
            fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
                Ok($name {
                    inner: U32Reader::read(reader)?,
                })
            }
        }

        impl Validate for $name {
            fn validate(&self) -> bool {
                self.inner.validate()
            }
        }
    };
}
