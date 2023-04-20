pub fn sha1_str(input: &str) -> String {
    let mut bytes = input.bytes().collect::<Vec<u8>>();
    let original_length = bytes.len();
    let bit_length = original_length * 8;

    // Step 1: Padding
    bytes.push(0b10000000);
    while bytes.len() % 64 != 56 {
        bytes.push(0);
    }

    // Step 2: Append length
    let length_bytes = bit_length.to_be_bytes();
    bytes.extend_from_slice(&length_bytes);

    // Step 3: Initialize variables
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xEFCDAB89;
    let mut h2: u32 = 0x98BADCFE;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xC3D2E1F0;

    // Step 4: Process message in 512-bit chunks
    for chunk in bytes.chunks(64) {
        let mut words = [0u32; 80];

        // Step 4.1: Break chunk into sixteen 32-bit big-endian words
        for i in 0..16 {
            let j = i * 4;
            words[i] = ((chunk[j] as u32) << 24) | ((chunk[j + 1] as u32) << 16) | ((chunk[j + 2] as u32) << 8) | (chunk[j + 3] as u32);
        }

        // Step 4.2: Extend sixteen 32-bit words into eighty 32-bit words
        for i in 16..80 {
            let tmp = words[i - 3] ^ words[i - 8] ^ words[i - 14] ^ words[i - 16];
            words[i] = tmp.rotate_left(1);
        }

        // Step 4.3: Initialize hash value for this chunk
        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        // Step 4.4: Main loop
        for i in 0..80 {
            let (f, k) = if i < 20 {
                ((b & c) | ((!b) & d), 0x5A827999)
            } else if i < 40 {
                (b ^ c ^ d, 0x6ED9EBA1)
            } else if i < 60 {
                ((b & c) | (b & d) | (c & d), 0x8F1BBCDC)
            } else {
                (b ^ c ^ d, 0xCA62C1D6)
            };

            let tmp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(words[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = tmp;
        }

        // Step 4.5: Add the hash of this chunk to the result so far
        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
    }

    // Step 5: Output the hash
    let mut result = String::new();
    result.push_str(&format!("{:08x}", h0));
    result.push_str(&format!("{:08x}", h1));
    result.push_str(&format!("{:08x}", h2));
    result.push_str(&format!("{:08x}", h3));
    result.push_str(&format!("{:08x}", h4));

    result
}

pub fn sha1(input: &[u8]) -> [u8; 20] {
    let mut h: [u32; 5] = [0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let mut bytes = input.to_vec();
    let bit_len = bytes.len() * 8;

    bytes.push(0x80);
    while bytes.len() % 64 != 56 {
        bytes.push(0);
    }

    let mut len_bytes = [0u8; 8];
    let len_bits = bit_len as u64;
    for i in 0..8 {
        len_bytes[7 - i] = ((len_bits >> (i * 8)) & 0xFF) as u8;
    }
    bytes.extend_from_slice(&len_bytes);

    for chunk in bytes.chunks(64) {
        let mut w = [0u32; 80];

        for i in 0..16 {
            w[i] = ((chunk[i * 4] as u32) << 24) | ((chunk[i * 4 + 1] as u32) << 16) | ((chunk[i * 4 + 2] as u32) << 8) | (chunk[i * 4 + 3] as u32);
        }

        for i in 16..80 {
            w[i] = w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16];
            w[i] = w[i].rotate_left(1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];

        for i in 0..80 {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                60..=79 => (b ^ c ^ d, 0xCA62C1D6),
                _ => unreachable!(),
            };

            let temp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }

    let mut result = [0u8; 20];
    for i in 0..5 {
        result[i * 4] = ((h[i] >> 24) & 0xFF) as u8;
        result[i * 4 + 1] = ((h[i] >> 16) & 0xFF) as u8;
        result[i * 4 + 2] = ((h[i] >> 8) & 0xFF) as u8;
        result[i * 4 + 3] = (h[i] & 0xFF) as u8;
    }
    result
}

pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hash = [0u8; 32];

    let mut h0: u32 = 0x6a09e667;
    let mut h1: u32 = 0xbb67ae85;
    let mut h2: u32 = 0x3c6ef372;
    let mut h3: u32 = 0xa54ff53a;
    let mut h4: u32 = 0x510e527f;
    let mut h5: u32 = 0x9b05688c;
    let mut h6: u32 = 0x1f83d9ab;
    let mut h7: u32 = 0x5be0cd19;

    let k: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    let mut padded_data = Vec::from(data);

    // Append padding bits and length
    padded_data.push(0x80);
    while (padded_data.len() * 8) % 512 != 448 {
        padded_data.push(0x00);
    }
    let data_len_bits = (data.len() as u64) * 8;
    padded_data.extend(&data_len_bits.to_be_bytes());

    for chunk in padded_data.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i * 4], chunk[i * 4 + 1], chunk[i * 4 + 2], chunk[i * 4 + 3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }

        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = (h0, h1, h2, h3, h4, h5, h6, h7);

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(k[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
        h5 = h5.wrapping_add(f);
        h6 = h6.wrapping_add(g);
        h7 = h7.wrapping_add(h);
    }

    hash.copy_from_slice(
        &[
            h0.to_be_bytes(),
            h1.to_be_bytes(),
            h2.to_be_bytes(),
            h3.to_be_bytes(),
            h4.to_be_bytes(),
            h5.to_be_bytes(),
            h6.to_be_bytes(),
            h7.to_be_bytes(),
        ]
        .concat()[..],
    );

    hash
}
