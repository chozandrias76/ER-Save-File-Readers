use crate::impl_attribute;

impl_attribute!(Unimplemented4);

mod tests {
  use super::*;
  use crate::impl_read_ok_and_err_test;

  impl_read_ok_and_err_test!(Unimplemented4, ok: vec![1], err: vec![]);
}
