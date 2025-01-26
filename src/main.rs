use base64::{Engine, engine::general_purpose::STANDARD};

#[derive(Debug)]
enum Passage {
    Undefined,
    Open,
    Wall,
}

#[derive(Debug)]
enum RadarItem {
    None,
    Ally,
    Enemy,
    Monster,
    Hint,
    Target,
}

impl RadarItem {
    fn to_bits(&self) -> u8 {
        match *self {
            RadarItem::None => 0b0000,
            RadarItem::Ally => 0b0001,
            RadarItem::Enemy => 0b0010,
            RadarItem::Monster => 0b0011,
            RadarItem::Hint => 0b0100,
            RadarItem::Target => 0b1000,
        }
    }
}

fn encode_passages(passages: &[Passage]) -> Vec<u8> {
    let mut bits = Vec::new();
    for passage in passages {
        let bit = match passage {
            Passage::Undefined => 0b00,
            Passage::Open => 0b01,
            Passage::Wall => 0b10,
        };
        bits.push(bit);
    }

    // Regrouper les bits par octets
    let mut result = Vec::new();
    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << (7 - i);
        }
        result.push(byte);
    }

    result
}

fn encode_radar_items(items: &[RadarItem]) -> Vec<u8> {
    let mut bits = Vec::new();
    for item in items {
        bits.push(item.to_bits());
    }

    // Regrouper les bits sur 8 bits (1 octet par cellule)
    let mut result = Vec::new();
    for chunk in bits.chunks(2) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << (6 - i * 2);
        }
        result.push(byte);
    }

    result
}

fn main() {
    // Exemple de passages
    let horizontal_passages = vec![
        Passage::Undefined, Passage::Open, Passage::Wall,
        Passage::Wall, Passage::Open, Passage::Undefined,
        Passage::Open, Passage::Wall, Passage::Undefined,
        Passage::Wall, Passage::Undefined, Passage::Undefined,
    ];

    let vertical_passages = vec![
        Passage::Undefined, Passage::Wall, Passage::Wall, Passage::Undefined,
        Passage::Wall, Passage::Open, Passage::Wall, Passage::Undefined,
        Passage::Wall, Passage::Undefined, Passage::Undefined, Passage::Undefined,
    ];

    // Exemple d'items sur le radar
    let radar_items = vec![
        RadarItem::None, RadarItem::Ally, RadarItem::Enemy,
        RadarItem::Monster, RadarItem::Hint, RadarItem::Target,
        RadarItem::None, RadarItem::Ally, RadarItem::None,
    ];

    // Encoder les passages
    let horizontal_encoded = encode_passages(&horizontal_passages);
    let vertical_encoded = encode_passages(&vertical_passages);

    // Encoder les items
    let items_encoded = encode_radar_items(&radar_items);

    // Concatenation des encodages
    let mut full_encoded = Vec::new();
    full_encoded.extend_from_slice(&horizontal_encoded);
    full_encoded.extend_from_slice(&vertical_encoded);
    full_encoded.extend_from_slice(&items_encoded);

    // Convertir en Base64 en utilisant la méthode `encode` de la crate `base64`
    let encoded_b64 = STANDARD.encode(&full_encoded);

    // Afficher la chaîne Base64 encodée
    println!("Encoded Base64 string: {}", encoded_b64);
}
