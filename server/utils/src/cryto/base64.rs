const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(input: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < input.len() {
        let byte1 = input[i];
        let byte2 = if i + 1 < input.len() { input[i + 1] } else { 0 };
        let byte3 = if i + 2 < input.len() { input[i + 2] } else { 0 };

        let index1 = byte1 >> 2;
        let index2 = ((byte1 & 0x03) << 4) | (byte2 >> 4);
        let index3 = ((byte2 & 0x0F) << 2) | (byte3 >> 6);
        let index4 = byte3 & 0x3F;

        result.push(BASE64_CHARS[index1 as usize] as char);
        result.push(BASE64_CHARS[index2 as usize] as char);
        result.push(if i + 1 < input.len() { BASE64_CHARS[index3 as usize] } else { b'=' } as char);
        result.push(if i + 2 < input.len() { BASE64_CHARS[index4 as usize] } else { b'=' } as char);

        i += 3;
    }

    result
}

pub fn decode(input: &str) -> Option<Vec<u8>> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let index1 = BASE64_CHARS.iter().position(|&x| x == input.as_bytes()[i])?;
        let index2 = BASE64_CHARS.iter().position(|&x| x == input.as_bytes()[i + 1])?;
        let index3 = if input.as_bytes()[i + 2] != b'=' {
            BASE64_CHARS.iter().position(|&x| x == input.as_bytes()[i + 2])?
        } else {
            0
        };
        let index4 = if input.as_bytes()[i + 3] != b'=' {
            BASE64_CHARS.iter().position(|&x| x == input.as_bytes()[i + 3])?
        } else {
            0
        };

        let byte1 = (index1 as u8) << 2 | (index2 as u8) >> 4;
        let byte2 = (index2 as u8) << 4 | (index3 as u8) >> 2;
        let byte3 = (index3 as u8) << 6 | index4 as u8;

        result.push(byte1);
        if input.as_bytes()[i + 2] != b'=' {
            result.push(byte2);
        }
        if input.as_bytes()[i + 3] != b'=' {
            result.push(byte3);
        }

        i += 4;
    }

    Some(result)
}
