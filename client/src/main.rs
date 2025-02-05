mod radar;
mod text;
//use text::{encode, decode};
mod maze;
use maze::{encode_maze, decode_maze};
//use std::net::TcpStream;
//use std::io::{Write, Read};
//use base64::{engine::general_purpose::STANDARD, Engine};
//use radar::{encode_passages, encode_radar_items, decode_passages, decode_radar_items, Passage, RadarItem};

fn generate_large_maze(nx: u16, ny: u16) -> (u16, u16, Vec<bool>, Vec<bool>) {
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


fn main() {

    //TODO edgar à enlever plus tard (test)
    // let horizontal_passages = vec![
    //     Passage::Undefined, Passage::Open, Passage::Wall,
    //     Passage::Wall, Passage::Open, Passage::Undefined,
    //     Passage::Open, Passage::Wall, Passage::Undefined,
    //     Passage::Wall, Passage::Undefined, Passage::Undefined,
    // ];

    // let vertical_passages = vec![
    //     Passage::Undefined, Passage::Wall, Passage::Wall, Passage::Undefined,
    //     Passage::Wall, Passage::Open, Passage::Wall, Passage::Undefined,
    //     Passage::Wall, Passage::Undefined, Passage::Undefined, Passage::Undefined,
    // ];

    // let radar_items = vec![
    //     RadarItem::None, RadarItem::Ally, RadarItem::Enemy,
    //     RadarItem::Monster, RadarItem::Hint, RadarItem::Target,
    //     RadarItem::None, RadarItem::Ally, RadarItem::None,
    // ];

    // let horizontal_encoded = encode_passages(&horizontal_passages);
    // let vertical_encoded = encode_passages(&vertical_passages);
    // let items_encoded = encode_radar_items(&radar_items);

    // let mut full_encoded = Vec::new();
    // full_encoded.extend_from_slice(&horizontal_encoded);
    // full_encoded.extend_from_slice(&vertical_encoded);
    // full_encoded.extend_from_slice(&items_encoded);

    // let encoded_b64 = STANDARD.encode(&full_encoded);
    // println!("Encoded Base64 string: {}", encoded_b64);

    // let decoded_bytes = STANDARD.decode(&encoded_b64).expect("Failed to decode base64");

    // let horizontal_size = (horizontal_passages.len() + 3) / 4;
    // let vertical_size = (vertical_passages.len() + 3) / 4;

    // let (horizontal_decoded, rest) = decoded_bytes.split_at(horizontal_size);
    // let (vertical_decoded, radar_decoded) = rest.split_at(vertical_size);

    // let horizontal_passages_decoded = decode_passages(horizontal_decoded, horizontal_passages.len());
    // let vertical_passages_decoded = decode_passages(vertical_decoded, vertical_passages.len());
    // let radar_items_decoded = decode_radar_items(radar_decoded, radar_items.len());

    // println!("Decoded horizontal passages: {:?}", horizontal_passages_decoded);
    // println!("Decoded vertical passages: {:?}", vertical_passages_decoded);
    // println!("Decoded radar items: {:?}", radar_items_decoded);

    let nx = 5; // Largeur du labyrinthe
    let ny = 5; // Hauteur du labyrinthe

    // Générer un labyrinthe 25x25
    let (nx, ny, horizontal_walls, vertical_walls) = generate_large_maze(nx, ny);

    // Encodage du labyrinthe
    let encoded_maze = encode_maze(nx, ny, &horizontal_walls, &vertical_walls);
    println!("Encoded Maze: {}", encoded_maze);

    // let encoded_maze = "AwADAAZgTy8=";
    // decode_maze(encoded_maze);

    // let data = b"Connais tu les 3 C Caca Clope Cafe";
    // let encoded = encode(data);
    // println!("Encoded: {}", encoded);

    // let test = "q29UBMfPCYb0DsbSzxmGmYbdienHy2eGq2XVCguGq2fMzq";

    // match decode(test) {
    //     Ok(decoded) => println!("Decoded: {:?}", String::from_utf8(decoded).unwrap()),
    //     Err(e) => println!("Error: {}", e),
    // }

}
