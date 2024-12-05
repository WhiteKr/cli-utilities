use std::fs;

use crate::CliResult;

pub fn execute(files: &[String]) -> CliResult<()> {
    for file in files {
        let content = fs::read_to_string(file)?;
        print!("{}", content);
    }
    Ok(())
}
