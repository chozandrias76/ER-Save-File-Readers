use std::io::{self, Read, Seek};

pub trait BinaryReadable: Sized {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self>;
}
