#[macro_export]
macro_rules! impl_read_ok_and_err_test {
    ($struct_name:ident, ok: $data_ok:expr, err: $data_err:expr) => {
        use std::io::Cursor;

        #[test]
        fn test_read_is_ok() {
            let data = $data_ok;
            let mut reader = Cursor::new(data);
            let result = $struct_name::read(&mut reader);
            assert!(result.is_ok());
        }

        #[test]
        fn test_read_is_err() {
            let data = $data_err;
            let mut reader = Cursor::new(data);
            let result = $struct_name::read(&mut reader);
            assert!(result.is_err());
        }
    };
}
