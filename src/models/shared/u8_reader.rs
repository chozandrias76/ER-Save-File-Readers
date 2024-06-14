use std::{
    fmt,
    io::{self, Read, Seek},
};

use crate::traits::binary_readable::BinaryReadable;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct U8Reader {
    pub data: (u8,),
}

impl Default for U8Reader {
    fn default() -> Self {
        Self {
            data: (u8::default(),),
        }
    }
}

impl fmt::Debug for U8Reader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "U8Reader(\n")?;
        write!(f, "{:02X}\u{2008}", self.data.0)?;
        write!(f, ")")
    }
}

impl BinaryReadable for U8Reader {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let mutable_default: &mut [u8; 1] = &mut Self::default().data.into();
        // let mutable_default = &mut [u8::default()];
        match reader.read_exact(mutable_default) {
            Ok(_) => Ok(U8Reader {
                data: (mutable_default[0],),
            }),
            Err(e) => {
                return Err(e);
            }
        }
    }
}
