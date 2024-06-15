use crate::impl_byte_array_readable;

impl_byte_array_readable!(Checksum, 0x10);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impl_read_ok_and_err_test;

    impl_read_ok_and_err_test!(Checksum, ok: vec![0; 0x10], err: vec![0; 0xf]);
}
