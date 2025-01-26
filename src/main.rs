use base64::{engine::general_purpose::STANDARD, Engine};

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

    fn from_bits(bits: u8) -> RadarItem {
        match bits {
            0b0000 => RadarItem::None,
            0b0001 => RadarItem::Ally,
            0b0010 => RadarItem::Enemy,
            0b0011 => RadarItem::Monster,
            0b0100 => RadarItem::Hint,
            0b1000 => RadarItem::Target,
            _ => RadarItem::None,
        }
    }
}

impl Passage {
    fn to_bits(&self) -> u8 {
        match *self {
            Passage::Undefined => 0b00,
            Passage::Open => 0b01,
            Passage::Wall => 0b10,
        }
    }

    fn from_bits(bits: u8) -> Passage {
        match bits {
            0b00 => Passage::Undefined,
            0b01 => Passage::Open,
            0b10 => Passage::Wall,
            _ => Passage::Undefined,
        }
    }
}

fn encode_passages(passages: &[Passage]) -> Vec<u8> {
    let mut bits = Vec::new();
    for passage in passages {
        bits.push(passage.to_bits());
    }

    let mut result = Vec::new();
    for chunk in bits.chunks(4) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << (6 - i * 2);
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

    let mut result = Vec::new();
    for chunk in bits.chunks(2) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << (4 - i * 4);
        }
        result.push(byte);
    }

    result
}

fn decode_passages(bytes: &[u8], expected_count: usize) -> Vec<Passage> {
    let mut passages = Vec::new();
    for &byte in bytes {
        for i in 0..4 {
            if passages.len() >= expected_count {
                break;
            }
            let passage_bits = (byte >> (6 - i * 2)) & 0b11;
            passages.push(Passage::from_bits(passage_bits));
        }
    }
    passages
}

fn decode_radar_items(bytes: &[u8], expected_count: usize) -> Vec<RadarItem> {
    let mut items = Vec::new();
    for &byte in bytes {
        for i in 0..2 {
            if items.len() >= expected_count {
                break;
            }
            let radar_bits = (byte >> (4 - i * 4)) & 0b1111;
            items.push(RadarItem::from_bits(radar_bits));
        }
    }
    items
}

fn main() {

    // TODO à enlever plus tard (test)
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

    let radar_items = vec![
        RadarItem::None, RadarItem::Ally, RadarItem::Enemy,
        RadarItem::Monster, RadarItem::Hint, RadarItem::Target,
        RadarItem::None, RadarItem::Ally, RadarItem::None,
    ];

    let horizontal_encoded = encode_passages(&horizontal_passages);
    let vertical_encoded = encode_passages(&vertical_passages);
    let items_encoded = encode_radar_items(&radar_items);

    let mut full_encoded = Vec::new();
    full_encoded.extend_from_slice(&horizontal_encoded);
    full_encoded.extend_from_slice(&vertical_encoded);
    full_encoded.extend_from_slice(&items_encoded);

    let encoded_b64 = STANDARD.encode(&full_encoded);
    println!("Encoded Base64 string: {}", encoded_b64);

    let decoded_bytes = STANDARD.decode(&encoded_b64).expect("Failed to decode base64");

    let horizontal_size = (horizontal_passages.len() + 3) / 4;
    let vertical_size = (vertical_passages.len() + 3) / 4;

    let (horizontal_decoded, rest) = decoded_bytes.split_at(horizontal_size);
    let (vertical_decoded, radar_decoded) = rest.split_at(vertical_size);

    let horizontal_passages_decoded = decode_passages(horizontal_decoded, horizontal_passages.len());
    let vertical_passages_decoded = decode_passages(vertical_decoded, vertical_passages.len());
    let radar_items_decoded = decode_radar_items(radar_decoded, radar_items.len());

    println!("Decoded horizontal passages: {:?}", horizontal_passages_decoded);
    println!("Decoded vertical passages: {:?}", vertical_passages_decoded);
    println!("Decoded radar items: {:?}", radar_items_decoded);
}
