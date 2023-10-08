use anyhow::Result;

fn count_bytes(input: &[u8]) -> Result<u64> {
    Ok(input.len().try_into()?)
}

fn count_lines(input: &[u8]) -> Result<u64> {
    let count: u64 = input.iter().filter(|c| **c == b'\n').count().try_into()?;
    Ok(count)
}

fn count_words(input: &[u8]) -> Result<u64> {
    fn is_whitespace(c: u8) -> bool {
        c == b'\r' || c == b'\n' || c == b' ' || c == b'\t'
    }

    let mut count = 0;

    let mut it = input.iter().peekable();

    while let Some(&c) = it.next() {
        if !is_whitespace(c) {
            count += 1;

            while it.next_if(|&c| !is_whitespace(*c)).is_some() {}
        }
    }

    Ok(count)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    static TEST_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.txt");

    #[test]
    fn counting_bytes_works() {
        assert_eq!(
            super::count_bytes("Hello world!".as_bytes()).expect("count bytes in string"),
            12
        );
        assert_eq!(
            super::count_bytes(&get_test_input()).expect("count bytes in file"),
            342_190
        );
    }

    #[test]
    fn counting_lines_works() {
        assert_eq!(
            super::count_lines("Hello world!".as_bytes()).expect("count lines in string"),
            0
        );
        assert_eq!(
            super::count_lines(&get_test_input()).expect("count lines in string"),
            7_145
        );
    }

    #[test]
    fn counting_words_works() {
        assert_eq!(
            super::count_words("Hello world!".as_bytes()).expect("count words in string"),
            2
        );
        assert_eq!(
            super::count_words(&get_test_input()).expect("count words in string"),
            58_164
        );
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
