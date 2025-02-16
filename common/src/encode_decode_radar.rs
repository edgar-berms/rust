//use base64::{engine::general_purpose::STANDARD, Engine};

#[derive(Debug)]
pub enum Passage {
    Undefined,
    Open,
    Wall,
}

#[derive(Debug)]
pub enum RadarItem {
    None,
    Ally,
    Enemy,
    Monster,
    Hint,
    Target,
}

impl RadarItem {
    pub fn to_bits(&self) -> u8 {
        match *self {
            RadarItem::None => 0b0000,
            RadarItem::Ally => 0b0001,
            RadarItem::Enemy => 0b0010,
            RadarItem::Monster => 0b0011,
            RadarItem::Hint => 0b0100,
            RadarItem::Target => 0b1000,
        }
    }

    pub fn from_bits(bits: u8) -> RadarItem {
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
    pub fn to_bits(&self) -> u8 {
        match *self {
            Passage::Undefined => 0b00,
            Passage::Open => 0b01,
            Passage::Wall => 0b10,
        }
    }

    pub fn from_bits(bits: u8) -> Passage {
        match bits {
            0b00 => Passage::Undefined,
            0b01 => Passage::Open,
            0b10 => Passage::Wall,
            _ => Passage::Undefined,
        }
    }
}

pub fn encode_passages(passages: &[Passage]) -> Vec<u8> {
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

pub fn encode_radar_items(items: &[RadarItem]) -> Vec<u8> {
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

pub fn decode_passages(bytes: &[u8], expected_count: usize) -> Vec<Passage> {
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

pub fn decode_radar_items(bytes: &[u8], expected_count: usize) -> Vec<RadarItem> {
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