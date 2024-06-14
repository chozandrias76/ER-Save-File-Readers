use std::io;

pub trait ByteArrayReadable: Sized {
  fn read<R: std::io::Read + std::io::Seek>(reader: &mut R) -> Result<Self, io::Error>
  where
      Self: Sized;
}