pub fn encode(data: &[u8]) -> String {
    let base32_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut result = String::new();

    let mut bits = 0;
    let mut bits_remaining = 0;

    for byte in data {
        bits |= (byte >> bits_remaining) as u32;
        bits_remaining += 8;

        while bits_remaining >= 5 {
            let index = bits & 0x1f;
            result.push(base32_alphabet.chars().nth(index as usize).unwrap());

            bits >>= 5;
            bits_remaining -= 5;
        }
    }

    if bits_remaining > 0 {
        let index = bits & 0x1f;
        result.push(base32_alphabet.chars().nth(index as usize).unwrap());
    }

    result
}

pub fn decode(data: &str) -> Result<Vec<u8>, &'static str> {
    let base32_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut result = Vec::new();

    let mut bits = 0;
    let mut bits_remaining = 0;

    for c in data.chars() {
        let index = base32_alphabet.find(c).ok_or("Invalid character")? as u32;
        bits |= index << bits_remaining;
        bits_remaining += 5;

        if bits_remaining >= 8 {
            result.push((bits & 0xff) as u8);
            bits >>= 8;
            bits_remaining -= 8;
        }
    }

    if bits_remaining > 0 {
        Err("Invalid input")
    } else {
        Ok(result)
    }
}
