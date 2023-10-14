use std::io::Read;

use anyhow::Result;

const READ_BUFFER_SIZE: usize = 512;

pub fn count_bytes(input: &mut impl Read) -> Result<u64> {
    let mut buf = [0; READ_BUFFER_SIZE];
    let mut count = 0;

    loop {
        let bytes_read = input.read(&mut buf)?;

        if bytes_read == 0 {
            return Ok(count.try_into()?);
        }

        count += bytes_read;
    }
}

pub fn count_lines(input: &mut impl Read) -> Result<u64> {
    let mut buf = [0; READ_BUFFER_SIZE];
    let mut count = 0;

    loop {
        let bytes_read = input.read(&mut buf)?;

        if bytes_read == 0 {
            return Ok(count.try_into()?);
        }

        count += buf.iter().take(bytes_read).filter(|&c| *c == b'\n').count();
    }
}

pub fn count_words(input: &mut impl Read) -> Result<u64> {
    fn is_whitespace(c: u8) -> bool {
        c == b'\r' || c == b'\n' || c == b' ' || c == b'\t'
    }

    let mut buf = [0; READ_BUFFER_SIZE];
    let mut count = 0;

    let mut in_whitespace = true;

    loop {
        let bytes_read = input.read(&mut buf)?;

        if bytes_read == 0 {
            return Ok(count);
        }

        for c in &buf[0..bytes_read] {
            if !is_whitespace(*c) && in_whitespace {
                count += 1;
                in_whitespace = false;
            } else if is_whitespace(*c) {
                in_whitespace = true;
            }
        }
    }
}

pub fn count_characters(input: &mut impl Read) -> Result<u64> {
    let mut buf = vec![];
    input.read_to_end(&mut buf)?;
    Ok(String::from_utf8(buf)?.chars().count().try_into()?)
}

#[cfg(test)]
mod test {
    static TEST_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.txt");

    #[test]
    fn counting_bytes_works() {
        {
            let mut input = "Hello world!".as_bytes();
            assert_eq!(
                super::count_bytes(&mut input).expect("count bytes in string"),
                12
            );
        }
        {
            let input = get_test_input();
            assert_eq!(
                super::count_bytes(&mut input.as_slice()).expect("count bytes in string"),
                342_190
            );
        }
    }

    #[test]
    fn counting_lines_works() {
        {
            let mut input = "Hello world!".as_bytes();
            assert_eq!(
                super::count_lines(&mut input).expect("count lines in string"),
                0
            );
        }
        {
            let input = get_test_input();
            assert_eq!(
                super::count_lines(&mut input.as_slice()).expect("count lines in string"),
                7_145
            );
        }
    }

    #[test]
    fn counting_words_works() {
        {
            let mut input = "Hello world!".as_bytes();
            assert_eq!(
                super::count_words(&mut input).expect("count words in string"),
                2
            );
        }
        {
            let input = get_test_input();
            assert_eq!(
                super::count_words(&mut input.as_slice()).expect("count words in string"),
                58_164
            );
        }
    }

    #[test]
    fn counting_characters_works() {
        {
            let mut input = "Hello world!".as_bytes();
            assert_eq!(
                super::count_characters(&mut input).expect("count characters in string"),
                12
            );
        }
        {
            let input = get_test_input();
            assert_eq!(
                super::count_characters(&mut input.as_slice()).expect("count characters in string"),
                339_292
            );
        }
    }

    fn get_test_input() -> Vec<u8> {
        use std::fs::File;
        use std::io::prelude::*;

        let mut contents = vec![];
        let mut f = File::open(TEST_FILE).expect(&format!("open file {}", TEST_FILE));
        f.read_to_end(&mut contents).expect("read file");
        contents
    }
}
