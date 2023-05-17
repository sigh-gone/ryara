use std::{fs::File, io::Read, iter::Peekable, str::Chars};

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

    #[test]
    pub fn try_get_tokens() {
        /*let numbers = vec![1, 2, 3, 4, 5];
        let mut iter = numbers.iter().peekable();

        while let Some(&num) = iter.peek() {
            println!("Peeked: {}", num);
            iter.next();
        }*/
        let smokey = String::from("    hello  white  space");
        let mut chars = smokey.chars().peekable();
        println!("{:?}", chars);
        let smoke = consume_while(chars, char::is_whitespace);
        println!("{:?}", smoke);
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

fn get_tokens() {
    let r = r"
        $try { 4a 4c 5f 60 9a }
    ";
}

fn consume_while<F>(mut stuff: Peekable<Chars>, condition: F) -> (String, Peekable<Chars>)
where
    F: Fn(char) -> bool,
{
    let mut result = String::new();
    while stuff.peek().map_or(false, |c| condition(*c)) {
        result.push(stuff.next().unwrap());
    }

    (result, stuff)
}
