use std::{fs::File, io::Read};

use hex::ToHex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn comparing_hex_files() {
        let smoke = read_file_to_hex("./hello.txt").unwrap();
        let smoke2 = read_file_to_hex("./hello2.txt").unwrap();

        let data = hex::decode(smoke).unwrap();
        let pattern = hex::decode(smoke2).unwrap();

        assert!(search_pattern(&data, &pattern).is_some());
    }
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
