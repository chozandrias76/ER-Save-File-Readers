mod read_from_file {
    use crate::traits::{binary_readable::BinaryReadable, validate::Validate};
    use std::{
        fs::File,
        io::{self, BufReader},
    };

    fn read_from_file<T: BinaryReadable + Validate>(path: &str) -> io::Result<T> {
        if !path.ends_with(".sl2") {
            panic!("Invalid file format: expected a file ending with .sl2");
        }

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let result = T::read(&mut reader);
        match result {
            Ok(data) => {
                assert_eq!(data.validate(), true, "Data validation failed");
                Ok(data)
            }
            Err(e) => Err(e),
        }
    }

    mod save_slot {
        use std::{
            fs::File,
            io::{self, BufReader},
        };

        use crate::traits::validate::Validate;

        impl crate::models::save_slot::checksum::Checksum {
            pub fn read_from_file(path: &str) -> io::Result<Self> {
                let result =
                    crate::testbed::read_from_file::read_from_file::read_from_file::<Self>(path);
                match result {
                    Ok(data) => {
                        assert_eq!(data.validate(), true, "Data validation failed");
                        Ok(data)
                    }
                    Err(e) => Err(e),
                }
            }
        }

        impl crate::models::save_slot::save_slot::SaveSlot {
            pub fn read_from_file(path: &str) -> io::Result<Self> {
                let result =
                    crate::testbed::read_from_file::read_from_file::read_from_file::<Self>(path);
                match result {
                    Ok(data) => {
                        assert_eq!(data.validate(), true, "Data validation failed");
                        Ok(data)
                    }
                    Err(e) => Err(e),
                }
            }
        }

        impl crate::models::save_slot::map_id::MapID {
            pub fn read_from_file(path: &str) -> io::Result<Self> {
                let result =
                    crate::testbed::read_from_file::read_from_file::read_from_file::<Self>(path);
                match result {
                    Ok(data) => {
                        assert_eq!(data.validate(), true, "Data validation failed");
                        Ok(data)
                    }
                    Err(e) => Err(e),
                }
            }
        }

        impl crate::models::save_slot::unk01::Unk01 {
            pub fn read_from_file(path: &str) -> io::Result<Self> {
                let result =
                    crate::testbed::read_from_file::read_from_file::read_from_file::<Self>(path);
                match result {
                    Ok(data) => {
                        assert_eq!(data.validate(), true, "Data validation failed");
                        Ok(data)
                    }
                    Err(e) => Err(e),
                }
            }
        }

        impl crate::models::save_slot::unk_24_bytes::Unk24Bytes {
            pub fn read_from_file(path: &str) -> io::Result<Self> {
                let file = File::open(path)?;
                let mut reader = BufReader::new(file);
                let result = Self::read(&mut reader);
                match result {
                    Ok(data) => {
                        assert_eq!(data.validate(), true, "Data validation failed");
                        Ok(data)
                    }
                    Err(e) => Err(e),
                }
            }
        }

        impl crate::models::save_slot::gaitem_handle_map::gaitem_handle_map::GAItemHandleMap {
            pub fn read_from_file(path: &str) -> io::Result<Self> {
                let result =
                    crate::testbed::read_from_file::read_from_file::read_from_file::<Self>(path);
                match result {
                    Ok(data) => {
                        assert_eq!(data.validate(), true, "Data validation failed");
                        Ok(data)
                    }
                    Err(e) => Err(e),
                }
            }
        }
        mod player_game_data {
            use std::io::{self};

            use crate::traits::validate::Validate;

            impl crate::models::save_slot::player_game_data::unk::Unk {
                pub fn read_from_file(path: &str) -> io::Result<Self> {
                    let result = crate::testbed::read_from_file::read_from_file::read_from_file::<
                        Self,
                    >(path);
                    match result {
                        Ok(data) => {
                            assert_eq!(data.validate(), true, "Data validation failed");
                            Ok(data)
                        }
                        Err(e) => Err(e),
                    }
                }
            }

            impl crate::models::save_slot::player_game_data::unk1::Unk1 {
                pub fn read_from_file(path: &str) -> io::Result<Self> {
                    let result = crate::testbed::read_from_file::read_from_file::read_from_file::<
                        Self,
                    >(path);
                    match result {
                        Ok(data) => {
                            assert_eq!(data.validate(), true, "Data validation failed");
                            Ok(data)
                        }
                        Err(e) => Err(e),
                    }
                }
            }

            impl crate::models::save_slot::player_game_data::health::Health {
                pub fn read_from_file(path: &str) -> io::Result<Self> {
                    let result = crate::testbed::read_from_file::read_from_file::read_from_file::<
                        Self,
                    >(path);
                    match result {
                        Ok(data) => {
                            assert_eq!(data.validate(), true, "Data validation failed");
                            Ok(data)
                        }
                        Err(e) => Err(e),
                    }
                }
            }

            impl crate::models::save_slot::player_game_data::max_health::MaxHealth {
                pub fn read_from_file(path: &str) -> io::Result<Self> {
                    let result = crate::testbed::read_from_file::read_from_file::read_from_file::<
                        Self,
                    >(path);
                    match result {
                        Ok(data) => {
                            assert_eq!(data.validate(), true, "Data validation failed");
                            Ok(data)
                        }
                        Err(e) => Err(e),
                    }
                }
            }

            impl crate::models::save_slot::player_game_data::max_base_health::MaxBaseHealth {
                pub fn read_from_file(path: &str) -> io::Result<Self> {
                    let result = crate::testbed::read_from_file::read_from_file::read_from_file::<
                        Self,
                    >(path);
                    match result {
                        Ok(data) => {
                            assert_eq!(data.validate(), true, "Data validation failed");
                            Ok(data)
                        }
                        Err(e) => Err(e),
                    }
                }
            }

            impl crate::models::save_slot::player_game_data::fp::FP {
                pub fn read_from_file(path: &str) -> io::Result<Self> {
                    let result = crate::testbed::read_from_file::read_from_file::read_from_file::<
                        Self,
                    >(path);
                    match result {
                        Ok(data) => {
                            assert_eq!(data.validate(), true, "Data validation failed");
                            Ok(data)
                        }
                        Err(e) => Err(e),
                    }
                }
            }

            impl crate::models::save_slot::player_game_data::max_fp::MaxFP {
                pub fn read_from_file(path: &str) -> io::Result<Self> {
                    let result = crate::testbed::read_from_file::read_from_file::read_from_file::<
                        Self,
                    >(path);
                    match result {
                        Ok(data) => {
                            assert_eq!(data.validate(), true, "Data validation failed");
                            Ok(data)
                        }
                        Err(e) => Err(e),
                    }
                }
            }

            impl crate::models::save_slot::player_game_data::base_max_fp::BaseMaxFP {
                pub fn read_from_file(path: &str) -> io::Result<Self> {
                    let result = crate::testbed::read_from_file::read_from_file::read_from_file::<
                        Self,
                    >(path);
                    match result {
                        Ok(data) => {
                            assert_eq!(data.validate(), true, "Data validation failed");
                            Ok(data)
                        }
                        Err(e) => Err(e),
                    }
                }
            }

            impl crate::models::save_slot::player_game_data::unk2::Unk2 {
                pub fn read_from_file(path: &str) -> io::Result<Self> {
                    let result = crate::testbed::read_from_file::read_from_file::read_from_file::<
                        Self,
                    >(path);
                    match result {
                        Ok(data) => {
                            assert_eq!(data.validate(), true, "Data validation failed");
                            Ok(data)
                        }
                        Err(e) => Err(e),
                    }
                }
            }
        }
    }
}
