use std::{
    ffi::OsString,
    fs::File,
    io::{stdin, BufReader, Read},
};

use anyhow::{anyhow, bail, Result};
use challenge_1_wc::*;

#[derive(Debug, Default)]
struct Args {
    bytes: bool,
    lines: bool,
    words: bool,
    characters: bool,
    file: Option<OsString>,
}

fn main() -> Result<()> {
    let args = {
        use lexopt::prelude::*;

        let mut args = Args::default();

        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Short('c') => args.bytes = true,
                Short('l') => args.lines = true,
                Short('w') => args.words = true,
                Short('m') => args.characters = true,
                Value(x) if args.file.is_none() => args.file = Some(x),
                _ => bail!(anyhow!(arg.unexpected())),
            }
        }
        args
    };

    let mut input: Box<dyn Read> = if let Some(ref p) = args.file {
        Box::new(BufReader::new(File::open(p)?))
    } else {
        Box::new(BufReader::new(stdin().lock()))
    };

    let count = if args.bytes {
        count_bytes(&mut input)
    } else if args.lines {
        count_lines(&mut input)
    } else if args.words {
        count_words(&mut input)
    } else if args.characters {
        count_characters(&mut input)
    } else {
        bail!("no arguments specified");
    };

    print!("{:>8}", count?);

    if let Some(p) = args.file {
        println!(" {}", p.to_string_lossy());
    } else {
        println!();
    }

    Ok(())
}
