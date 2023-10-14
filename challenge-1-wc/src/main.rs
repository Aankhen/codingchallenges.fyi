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

    // This is a bit of a hack to abstract over `impl Read`.
    fn do_count(args: &Args, input: &mut impl Read) -> Result<u64> {
        if args.bytes {
            count_bytes(input)
        } else if args.lines {
            count_lines(input)
        } else if args.words {
            count_words(input)
        } else if args.characters {
            count_characters(input)
        } else {
            bail!("no arguments specified");
        }
    }

    let count = if let Some(ref p) = args.file {
        let mut reader = BufReader::new(File::open(p)?);
        do_count(&args, &mut reader)
    } else {
        let mut reader = BufReader::new(stdin().lock());
        do_count(&args, &mut reader)
    }?;

    print!("{:>8}", count);

    if let Some(p) = args.file {
        println!(" {}", p.to_string_lossy());
    } else {
        println!();
    }

    Ok(())
}
