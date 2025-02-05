mod radar;
mod text;
use text::{encode, decode};
mod maze;
use maze::{encode_maze, decode_maze};
use std::net::TcpStream;
use std::io::{Write, Read};
use base64::{engine::general_purpose::STANDARD, Engine};
use radar::{encode_passages, encode_radar_items, decode_passages, decode_radar_items, Passage, RadarItem};

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

    // let nx = 3;  
    // let ny = 3;

    // // Murs horizontaux pour le labyrinthe 2x2 = 8 murs
    // // Murs verticaux pour le labyrinthe 2x2 = 8 murs
    // // Murs horizontaux pour le labyrinthe 3x3 = 16 murs
    // // Murs verticaux pour le labyrinthe 3x3 = 16 murs

    // let horizontal_walls = vec![
    //     false, false, false, false,
    //     false, true, true, false,
    //     false, true, true, false,
    //     false, false, false, false,
    // ];

    // let vertical_walls = vec![
    //     false, true, false, false,
    //     true, true, true, true,
    //     false, false, true, false,
    //     true, true, true, true,
    // ];

    // let encoded_maze = encode_maze(nx, ny, &horizontal_walls, &vertical_walls);
    // println!("Encoded Maze: {}", encoded_maze);

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

    // Adresse et port du serveur
    let ip = "127.0.0.1";  // localhost
    let port = 8778;        // port du serveur

    // Essayer de se connecter au serveur
    let mut stream = match TcpStream::connect(format!("{}:{}", ip, port)) {
        Ok(stream) => stream,
        Err(_) => {
            eprintln!("Impossible de se connecter au serveur sur {}:{}", ip, port);
            return;
        }
    };

    println!("Connecté au serveur sur {}:{}", ip, port);

    // Message à envoyer au serveur
    let message = "Salut serveur !";

    // Envoi du message
    match stream.write_all(message.as_bytes()) {
        Ok(_) => println!("Message envoyé : {}", message),
        Err(_) => {
            eprintln!("Erreur lors de l'envoi du message");
            return;
        }
    }

    // Buffer pour lire la réponse du serveur
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let response = String::from_utf8_lossy(&buffer[..size]);
            println!("Réponse du serveur : {}", response);
        }
        Err(_) => {
            eprintln!("Erreur de lecture de la réponse du serveur");
        }
    }
}
