pub fn convert_endian(string: &str) -> String {
    let mut new_string = String::new();
    let mut prev_char = ' ';
    for (i, curr_char) in string.chars().rev().enumerate() {
        if i % 2 == 0 {
            prev_char = curr_char;
            continue;
        }
        new_string.push(curr_char);
        new_string.push(prev_char);
    }
    return new_string;
}

pub fn bytes_to_u64(bytes: &Vec<u8>) -> u64 {
    if bytes.len() > 8 {
        panic!("Input exceeds u64 size")
    }
    let mut varint = 0;
    for byte in bytes {
        varint <<= 8;
        varint |= *byte as u64;
    }
    return varint;
}

