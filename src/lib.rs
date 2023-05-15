use std::{fs, io};

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
