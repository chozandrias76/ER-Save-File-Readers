use std::io::{self, Read, Seek};

use crate::impl_u8_readable;

impl_u8_readable!(Attribute);

#[cfg(test)]
mod tests {
    use io::Cursor;

    use super::*;

    #[test]
    fn test_can_read() {
        let data = [0x0];
        let mut reader = io::BufReader::new(Cursor::new(&data));

        let attribute = Attribute::read(&mut reader);
        assert_eq!(attribute.is_ok(), true);
    }
}
