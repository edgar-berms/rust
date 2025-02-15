use base64::{engine::general_purpose::STANDARD, Engine};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

pub fn encode_maze(maze_str: &str) -> String {
    let bytes = maze_str.as_bytes();
    STANDARD.encode(bytes)
}

pub fn decode_maze(encoded: &str) -> String {
    let decoded_bytes = STANDARD.decode(encoded).expect("Erreur de décodage Base64");
    String::from_utf8(decoded_bytes).expect("Erreur de conversion en UTF-8")
}