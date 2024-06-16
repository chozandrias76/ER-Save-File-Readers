use crate::impl_attribute;

impl_attribute!(BodyHair);

mod tests {
  use super::*;
  use crate::impl_read_ok_and_err_test;

  impl_read_ok_and_err_test!(BodyHair, ok: vec![1], err: vec![]);
}
