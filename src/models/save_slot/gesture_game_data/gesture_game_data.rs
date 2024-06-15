use std::{
    fmt,
    io::{self, Read, Seek},
};

use crate::traits::binary_readable::BinaryReadable;

use super::gesture::Gesture;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct GestureGameData {
    pub gestures: Vec<Gesture>,
}

impl Default for GestureGameData {
    fn default() -> Self {
        Self {
            gestures: vec![Gesture::default(); 0x40],
        }
    }
}

impl fmt::Debug for GestureGameData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GestureGameData(/n")?;
        write!(f, "data: {:?}", self.gestures)?;
        write!(f, "/n)")
    }
}

impl BinaryReadable for GestureGameData {
    fn read<R: Read + Seek>(reader: &mut R) -> io::Result<Self> {
        let gestures: Vec<Gesture> = (0..0x40)
            .into_iter()
            .map(|_| match Gesture::read(reader) {
                Ok(gesture) => gesture,
                Err(e) => {
                    eprintln!("Error reading gesture: {}", e);
                    Gesture::default()
                }
            })
            .collect();
        return Ok(GestureGameData { gestures });
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use io::BufReader;

    use super::*;

    #[test]
    fn test_read_gesture_game_data() {
        let expected_bytes = [
            0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x06, 0x00,
            0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x00,
            0x0F, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x14, 0x00,
            0x00, 0x00, 0x29, 0x00, 0x00, 0x00, 0x2A, 0x00, 0x00, 0x00, 0x2C, 0x00, 0x00, 0x00,
            0x2E, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x3C, 0x00,
            0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x52, 0x00, 0x00, 0x00, 0x65, 0x00, 0x00, 0x00,
            0x66, 0x00, 0x00, 0x00, 0x68, 0x00, 0x00, 0x00, 0x6A, 0x00, 0x00, 0x00, 0x6C, 0x00,
            0x00, 0x00, 0x6E, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x8D, 0x00, 0x00, 0x00,
            0x8E, 0x00, 0x00, 0x00, 0x90, 0x00, 0x00, 0x00, 0x92, 0x00, 0x00, 0x00, 0xA0, 0x00,
            0x00, 0x00, 0xB4, 0x00, 0x00, 0x00, 0xB6, 0x00, 0x00, 0x00, 0xB9, 0x00, 0x00, 0x00,
            0xBA, 0x00, 0x00, 0x00, 0xBC, 0x00, 0x00, 0x00, 0xBE, 0x00, 0x00, 0x00, 0xC0, 0x00,
            0x00, 0x00, 0xC2, 0x00, 0x00, 0x00, 0xC4, 0x00, 0x00, 0x00, 0xC8, 0x00, 0x00, 0x00,
            0xCA, 0x00, 0x00, 0x00, 0xCC, 0x00, 0x00, 0x00, 0xCE, 0x00, 0x00, 0x00, 0xD0, 0x00,
            0x00, 0x00, 0xD2, 0x00, 0x00, 0x00, 0xD4, 0x00, 0x00, 0x00, 0xD8, 0x00, 0x00, 0x00,
            0xDA, 0x00, 0x00, 0x00, 0xDC, 0x00, 0x00, 0x00, 0xFE, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF,
            0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF,
            0xFE, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF,
            0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFF,
            0xFE, 0xFF, 0xFF, 0xFF,
        ];
        let mut reader = BufReader::new(
            File::open("testdata/vagabond/save_slots/0/gesture_game_data.sl2").unwrap(),
        );
        match GestureGameData::read(&mut reader) {
            Ok(gestures) => {
                assert_eq!(
                    expected_bytes.to_vec(),
                    gestures
                        .gestures
                        .into_iter()
                        .flat_map(|g| g.data.data.to_le_bytes())
                        .collect::<Vec<u8>>()
                )
            }
            Err(e) => {
                eprintln!("Error reading gesture: {}", e);
            }
        }
    }
}
