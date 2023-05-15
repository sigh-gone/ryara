#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::{Bytes, Read};
    use std::time::{Duration, Instant};

    #[test]
    fn it_works() {
        let file = File::open("hello.txt").expect("Cannot read file.");
        //let bytes = file.bytes();
        let mut buf = BufReader::new(file);

        for byte in buf.bytes() {
            if byte.unwrap() == 101 {
                println!("true");
            }
        }

        //bytes.find("predicate")
        assert!(false);
    }
}
