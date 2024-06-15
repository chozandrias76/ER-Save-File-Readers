use crate::impl_model_readable;

impl_model_readable!(Hair);
#[cfg(test)]
mod tests {
    use super::*;
    use crate::impl_read_ok_and_err_test;

    impl_read_ok_and_err_test!(Hair, ok: vec![0; 4], err: vec![0; 3]);
}
