#[macro_export]
macro_rules! impl_u8_readable {
    ($name:ident) => {
        use crate::models::shared::u8_reader::U8Reader;
        use crate::BinaryReadable;
        use std::io::{self, Read, Seek};
        use std::{
            fmt,
            ops::{Deref, DerefMut},
        };

        type Reader = U8Reader;

        #[derive(serde::Deserialize, serde::Serialize, Clone)]
        pub struct $name {
            pub data: Reader,
        }

        impl Default for $name {
            fn default() -> Self {
                $name {
                    data: Reader::default(),
                }
            }
        }

        // Implement Deref trait for $name
        impl Deref for $name {
            type Target = Reader;

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
        impl BinaryReadable for $name {
            fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, io::Error> {
                match Reader::read(reader) {
                    Ok(data) => Ok($name { data }),
                    Err(e) => Err(e),
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_reading_gives_ok() {
                let mut reader = std::io::Cursor::new(vec![0; 1]);
                let result = $name::read(&mut reader);
                assert!(result.is_ok());
            }
        }
    };
}
