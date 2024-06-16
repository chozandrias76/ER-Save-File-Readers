use crate::impl_attribute;

impl_attribute!(EyeshadowUpperColorR);

mod tests {
  use super::*;
  use crate::impl_read_ok_and_err_test;

  impl_read_ok_and_err_test!(EyeshadowUpperColorR, ok: vec![1], err: vec![]);
}
