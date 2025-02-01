use base64::{engine::general_purpose::STANDARD, Engine};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

pub fn encode_maze(nx: u16, ny: u16, horizontal_walls: &[bool], vertical_walls: &[bool]) -> String {
    let mut data = Vec::new();

    data.extend_from_slice(&nx.to_le_bytes());
    data.extend_from_slice(&ny.to_le_bytes());

    fn pack_bits(bits: &[bool]) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut byte = 0u8;
        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                byte |= 1 << (7 - (i % 8));
            }
            if i % 8 == 7 {
                bytes.push(byte);
                byte = 0;
            }
        }
        if bits.len() % 8 != 0 {
            bytes.push(byte);
        }
        bytes
    }

    data.extend_from_slice(&pack_bits(horizontal_walls));
    data.extend_from_slice(&pack_bits(vertical_walls));

    STANDARD.encode(&data)
}

pub fn decode_maze(encoded: &str) {
    let decoded_bytes = STANDARD.decode(encoded).unwrap();
    let mut cursor = Cursor::new(&decoded_bytes);

    let nx = cursor.read_u16::<LittleEndian>().unwrap();
    let ny = cursor.read_u16::<LittleEndian>().unwrap();

    println!("Labyrinthe de taille: {} x {}", nx, ny);

    let h_walls_size = ((ny + 1) * nx + 7) / 8;
    let v_walls_size = ((nx + 1) * ny + 7) / 8;

    let mut h_walls_bytes = vec![0u8; h_walls_size as usize];
    let mut v_walls_bytes = vec![0u8; v_walls_size as usize];

    cursor.read_exact(&mut h_walls_bytes).unwrap();
    cursor.read_exact(&mut v_walls_bytes).unwrap();

    let h_walls = decode_wall_bits(&h_walls_bytes);
    let v_walls = decode_wall_bits(&v_walls_bytes);

    println!("Murs horizontaux: {:?}", h_walls);
    println!("Murs verticaux: {:?}", v_walls);
}

fn decode_wall_bits(walls_bytes: &[u8]) -> Vec<u8> {
    let mut decoded = Vec::new();
    for &byte in walls_bytes {
        for i in 0..8 {
            decoded.push(if (byte & (1 << (7 - i))) != 0 { 1 } else { 0 });
        }
    }
    decoded
}
