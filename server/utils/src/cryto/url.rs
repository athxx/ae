// Encode a string as a URL-encoded string
pub fn encode(s: &str) -> String {
    let mut encoded = String::new();
    for byte in s.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(*byte as char);
            }
            _ => {
                encoded.push('%');
                encoded.push_str(&format!("{:02X}", byte));
            }
        }
    }
    encoded
}

// Decode a URL-encoded string into a normal string
pub fn decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut decoded = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' if i + 2 < bytes.len() => {
                let byte = u8::from_str_radix(&s[i + 1..i + 3], 16).unwrap_or(0);
                decoded.push(byte);
                i += 3;
            }
            _ => {
                decoded.push(bytes[i]);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&decoded).into_owned()
}
