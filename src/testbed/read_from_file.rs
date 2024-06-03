pub mod read_from_file {
    pub mod save_slot {
        pub mod player_game_data {
            use std::{fs::File, io::{self, BufReader}};

            use crate::traits::binary_readable::BinaryReadable;

            impl crate::models::save_slot::player_game_data::unk::Unk {
                pub fn read_from_file(path: &str) -> io::Result<crate::models::save_slot::player_game_data::unk::Unk> {
                  let file = File::open(path)?;
                  let mut reader = BufReader::new(file);
                  crate::models::save_slot::player_game_data::unk::Unk::read(&mut reader)
                }
            }
        }
    }
}
