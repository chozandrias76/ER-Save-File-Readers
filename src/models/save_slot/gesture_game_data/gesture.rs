use crate::impl_u32_readable;

impl_u32_readable!(Gesture);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impl_read_ok_and_err_test;

    impl_read_ok_and_err_test!(Gesture, ok: [
      0x01, 0x00, 0x00, 0x00
  ], err: vec![0; 3]);
}
