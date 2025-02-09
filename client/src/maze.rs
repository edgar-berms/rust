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

pub fn decode_wall_bits(walls_bytes: &[u8]) -> Vec<u8> {
    let mut decoded = Vec::new();
    for &byte in walls_bytes {
        for i in 0..8 {
            decoded.push(if (byte & (1 << (7 - i))) != 0 { 1 } else { 0 });
        }
    }
    decoded
}

pub fn generate_large_maze(nx: u16, ny: u16) -> (u16, u16, Vec<bool>, Vec<bool>) {
    // Création des murs horizontaux et verticaux
    let mut horizontal_walls = vec![false; (ny + 1) as usize * nx as usize];
    let mut vertical_walls = vec![false; (nx + 1) as usize * ny as usize];

    // Remplissage des murs horizontaux (chaque ligne a un mur au bord)
    for x in 0..nx {
        horizontal_walls[x as usize] = true; // Haut du labyrinthe
        horizontal_walls[((ny) * nx + x) as usize] = true; // Bas du labyrinthe
    }

    // Remplissage des murs verticaux (chaque colonne a un mur au bord)
    for y in 0..ny {
        vertical_walls[(y * (nx + 1)) as usize] = true; // Mur de gauche
        vertical_walls[(y * (nx + 1) + nx) as usize] = true; // Mur de droite
    }

    // Ajout de murs internes de manière aléatoire ou structurée (exemple simple ici)
    for x in 1..nx {
        for y in 1..ny {
            // Ajout de murs verticaux et horizontaux à des endroits choisis
            if (x + y) % 2 == 0 {
                horizontal_walls[(y * nx + x) as usize] = true;
                vertical_walls[(y * (nx + 1) + x) as usize] = true;
            }
        }
    }

    (nx, ny, horizontal_walls, vertical_walls)
}