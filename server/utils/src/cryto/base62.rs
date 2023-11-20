const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn encode(input: u64) -> String {
    let mut result = String::new();
    let mut n = input;
    while n > 0 {
        let rem = (n % 62) as usize;
        n /= 62;
        result.push(CHARSET[rem] as char);
    }
    result.chars().rev().collect()
}

pub fn decode(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut pow = 1;
    for ch in input.chars().rev() {
        let index = CHARSET.iter().position(|&x| x == ch as u8)?;
        result += index as u64 * pow;
        pow *= 62;
    }
    Some(result)
}
