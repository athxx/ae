use rand::Rng;

pub fn custom(size: usize) -> String {
    let alphabet: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-";
    let alphabet_size = alphabet.len();
    let mut rng = rand::thread_rng();
    let mut nanoid = String::with_capacity(size);

    for _ in 0..size {
        let index = rng.gen_range(0..alphabet_size);
        let char = alphabet[index] as char;
        nanoid.push(char);
    }

    nanoid
}

pub fn new() -> String {
    custom(21)
}
