/// usage
// Encode a TLV item
/*
let tag = 0x01;
let value = b"hello world";
let encoded = tlv_encode(tag, value);
println!("{:?}", encoded);

// Decode a TLV item
let data = encoded.as_slice();
if let Some((tag, value)) = tlv_decode(data) {
    println!("Tag: {:02X}", tag);
    println!("Value: {:?}", value);
}
*/

// Encode a tag-length-value (TLV) item
pub fn tlv_encode(tag: u8, value: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::new();
    encoded.push(tag);
    encoded.push(value.len() as u8);
    encoded.extend(value);
    encoded
}

// Decode a tag-length-value (TLV) item
pub fn tlv_decode(data: &[u8]) -> Option<(u8, &[u8])> {
    if data.len() < 2 {
        return None;
    }
    let tag = data[0];
    let len = data[1] as usize;
    if len > data.len() - 2 {
        return None;
    }
    Some((tag, &data[2..2 + len]))
}

/*
    // Encode an LV item
    let value = b"hello";
    let encoded = lv_encode(value);
    println!("{:?}", encoded);

    // Decode an LV item
    let data = encoded.as_slice();
    if let Some(value) = lv_decode(data) {
        println!("Value: {:?}", value);
    }
*/

// Encode a length-value (LV) item
pub fn lv_encode(value: &[u8]) -> Vec<u8> {
    let mut encoded = Vec::new();
    encoded.extend((value.len() as u16).to_be_bytes().iter());
    encoded.extend(value);
    encoded
}

// Decode a length-value (LV) item
pub fn lv_decode(data: &[u8]) -> Option<&[u8]> {
    if data.len() < 2 {
        return None;
    }
    let len = u16::from_be_bytes([data[0], data[1]]) as usize;
    if len > data.len() - 2 {
        return None;
    }
    Some(&data[2..2 + len])
}
