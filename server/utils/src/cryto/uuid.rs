use rand::RngCore;
use std::fmt;

#[derive(Copy, Clone)]
pub struct UuidV4 {
    pub bytes: [u8; 16],
}

impl fmt::Display for UuidV4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex_chars = "0123456789abcdef".chars().collect::<Vec<char>>();
        let mut s = String::new();
        for byte in &self.bytes {
            s.push(hex_chars[(byte >> 4) as usize]);
            s.push(hex_chars[(byte & 0xf) as usize]);
        }
        write!(f, "{}-{}-{}-{}-{}", &s[0..8], &s[8..12], &s[12..16], &s[16..20], &s[20..32])
    }
}

impl UuidV4 {
    pub fn new() -> UuidV4 {
        let mut bytes = [0u8; 16];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut bytes);

        // Set version to 4 (randomly generated UUID)
        bytes[6] = (bytes[6] & 0x0f) | 0x40;

        // Set variant to RFC 4122
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        UuidV4 { bytes }
    }
}

pub fn v4() -> String {
    UuidV4::new().to_string()
}
