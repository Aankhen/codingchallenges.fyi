use std::{
    ffi::OsString,
    fs::File,
    io::{stdin, BufReader, Read},
    process::ExitCode,
};

use anyhow::{anyhow, bail, Result};

#[derive(Debug, Default)]
struct Args {
    file: Option<OsString>,
}

fn main() -> Result<ExitCode> {
    let args = {
        use lexopt::prelude::*;

        let mut args = Args::default();

        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Value(x) if args.file.is_none() => args.file = Some(x),
                _ => bail!(anyhow!(arg.unexpected())),
            }
        }
        args
    };

    fn do_parse(_args: &Args, input: &mut impl Read) -> Result<bool> {
        match challenge_2_json_parser::parse_json(input) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    let parsed = if let Some(ref p) = args.file {
        let mut reader = BufReader::new(File::open(p)?);
        do_parse(&args, &mut reader)
    } else {
        let mut reader = BufReader::new(stdin().lock());
        do_parse(&args, &mut reader)
    }?;

    if parsed {
        Ok(ExitCode::SUCCESS)
    } else {
        Ok(ExitCode::FAILURE)
    }
}
