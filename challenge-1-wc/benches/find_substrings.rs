#![feature(test)]

use std::fs::File;
use std::io::BufReader;

extern crate test;
use test::{black_box, Bencher};

static TEST_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.txt");
static TEST_TEXT: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.txt"));

#[bench]
fn searching_without_memchr(b: &mut Bencher) {
    b.iter(|| {
        for _ in 1..10 {
            let mut reader = BufReader::new(TEST_TEXT);

            let _ = black_box(challenge_1_wc::count_lines(&mut reader));
        }
    });
}

#[bench]
fn searching_with_memchr(b: &mut Bencher) {
    b.iter(|| {
        for _ in 1..10 {
            let mut reader = BufReader::new(TEST_TEXT);

            let _ = black_box(challenge_1_wc::count_lines_memchr(&mut reader));
        }
    });
}

#[bench]
fn searching_file_without_memchr(b: &mut Bencher) {
    b.iter(|| {
        for _ in 1..10 {
            let mut reader = BufReader::new(File::open(&TEST_FILE).unwrap());

            let _ = black_box(challenge_1_wc::count_lines(&mut reader));
        }
    });
}

#[bench]
fn searching_file_with_memchr(b: &mut Bencher) {
    b.iter(|| {
        for _ in 1..10 {
            let mut reader = BufReader::new(File::open(&TEST_FILE).unwrap());

            let _ = black_box(challenge_1_wc::count_lines_memchr(&mut reader));
        }
    });
}
