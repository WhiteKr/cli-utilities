use crate::CliResult;

pub fn execute(text: &str) -> CliResult<()> {
    println!("{}", text);
    Ok(())
}
