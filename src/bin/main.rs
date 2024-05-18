fn main() {}

#[cfg(test)]
mod tests {
    use er_save_file_readers::models::save_slot::checksum::Checksum;
    use er_save_file_readers::traits::binary_readable::BinaryReadable;
    use std::fs::File;
    use std::io::{self, BufReader};

    fn read_checksum_from_file<T: BinaryReadable>(path: &str) -> io::Result<Checksum> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Checksum::read(&mut reader)
    }

    #[test]
    fn test_read_save_slot_checksum() {
        let expected_checksum = vec![
            10, 144, 247, 6, 252, 237, 233, 82, 77, 22, 69, 110, 236, 58, 252, 136,
        ];
        let checksum_data =
            read_checksum_from_file::<Checksum>("testdata/vagabond/save_slots/0/checksum.sl2")
                .expect("data should be present");

        for (index, &val) in checksum_data.data.iter().enumerate() {
            assert_eq!(val, expected_checksum[index])
        }
    }
}
