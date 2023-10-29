use std::io::Read;

use anyhow::{bail, Result};

pub fn parse_json(input: &mut impl Read) -> Result<()> {
    let mut buf = Vec::new();
    input.read_to_end(&mut buf)?;
    if &buf == b"{}" {
        Ok(())
    } else {
        bail!("invalid")
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::path::PathBuf;

    use super::parse_json;

    static TEST_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests");

    #[test]
    fn parsing_works() {
        let mut valid_file = get_file("step1", "valid.json");
        assert!(
            parse_json(&mut valid_file).is_ok(),
            "valid JSON must be parsed"
        );

        let mut invalid_file = get_file("step1", "invalid.json");
        assert!(
            parse_json(&mut invalid_file).is_err(),
            "invalid JSON must fail to be parsed"
        );
    }

    fn get_file(step: &str, filename: &str) -> File {
        let path = PathBuf::from(TEST_DIR).join(step).join(filename);
        File::open(&path).expect(&format!(
            "open file {:?}/{:?} = {}",
            step,
            filename,
            path.display()
        ))
    }
}
