const B64_ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/";

pub fn encode(input: &[u8]) -> String {
    let mut output = String::new();
    let mut buffer = 0u32;
    let mut bits_filled = 0;

    for &byte in input {
        buffer = (buffer << 8) | (byte as u32);
        bits_filled += 8;

        while bits_filled >= 6 {
            let index = (buffer >> (bits_filled - 6)) & 0b111111;
            output.push(B64_ALPHABET[index as usize] as char);
            bits_filled -= 6;
        }
    }

    if bits_filled > 0 {
        let index = (buffer << (6 - bits_filled)) & 0b111111;
        output.push(B64_ALPHABET[index as usize] as char);
    }

    output
}

pub fn decode(input: &str) -> Result<Vec<u8>, &'static str> {
    if input.len() % 4 == 1 {
        return Err("Taille invalide : la longueur ne peut pas être 4n+1.");
    }

    let mut output = Vec::new();
    let mut buffer = 0u32;
    let mut bits_filled = 0;

    for c in input.chars() {
        let value = match B64_ALPHABET.iter().position(|&x| x as char == c) {
            Some(v) => v as u32,
            None => return Err("Caractère non autorisé détecté."),
        };

        buffer = (buffer << 6) | value;
        bits_filled += 6;

        if bits_filled >= 8 {
            output.push((buffer >> (bits_filled - 8)) as u8);
            bits_filled -= 8;
        }
    }

    Ok(output)
}