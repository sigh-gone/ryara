use std::{
    fs::{self, File},
    io::{self, Read},
};

use hex::ToHex;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::{Bytes, Read};
    use std::time::{Duration, Instant};

    #[test]
    pub fn new_try() {
        let file_path = "./hello.txt";
        let pattern: &[u8] = &[104, 101, 108];

        match search_pattern_in_file(file_path, pattern) {
            Ok(Some(position)) => println!("Pattern found at position: {}", position),
            Ok(None) => println!("Pattern not found."),
            Err(error) => eprintln!("Error: {}", error),
        }
        assert!(false);
    }

    #[test]
    pub fn new_try2() {
        let data = &[
            0x6A, 0x40, 0x68, 0x00, 0x30, 0x00, 0x00, 0x6A, 0x14, 0x8D, 0x91,
        ];
        let pattern = &[
            0x8D, 0x4D, 0xB0, 0x2B, 0xC1, 0x83, 0xC0, 0x27, 0x99, 0x6A, 0x4E, 0x59, 0xF7, 0xF9,
        ];

        if let Some(position) = search_hex_bytes(data, pattern) {
            println!("Pattern found at position: {}", position);
        } else {
            println!("Pattern not found");
        }
    }

    #[test]
    pub fn new_try3() {
        let hex_string = "6865".to_owned();
        let pattern = hex::decode(hex_string).unwrap();
        let smoke = read_file_to_hex("./hello.txt").unwrap();
        println!("{:?}", smoke);
        let data = hex::decode(smoke).unwrap();

        if let Some(offset) = search_pattern(&data, &pattern) {
            println!("Pattern found at offset: {}", offset);
        } else {
            println!("Pattern not found");
        }
        assert!(false);
    }
}
fn search_pattern_in_file(file_path: &str, pattern: &[u8]) -> io::Result<Option<usize>> {
    let bytes = fs::read(file_path)?;
    let pattern_len = pattern.len();
    let bytes_len = bytes.len();

    for i in 0..=(bytes_len - pattern_len) {
        if bytes[i..i + pattern_len] == *pattern {
            return Ok(Some(i));
        }
    }

    Ok(None)
}

fn search_hex_bytes(data: &[u8], pattern: &[u8]) -> Option<usize> {
    data.windows(pattern.len())
        .position(|window| window == pattern)
}

fn read_file_to_hex(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer.encode_hex::<String>())
}

fn search_pattern(data: &[u8], pattern: &[u8]) -> Option<usize> {
    let pattern_len = pattern.len();
    let data_len = data.len();

    if pattern_len > data_len {
        return None;
    }

    for i in 0..=data_len - pattern_len {
        if data[i..i + pattern_len] == *pattern {
            return Some(i);
        }
    }

    None
}
