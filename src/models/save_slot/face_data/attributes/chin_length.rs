use crate::impl_attribute;

impl_attribute!(ChinLength);

mod tests {
  use super::*;
  use crate::impl_read_ok_and_err_test;

  impl_read_ok_and_err_test!(ChinLength, ok: vec![1], err: vec![]);
}
