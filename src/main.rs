use cli_utilities::{CliResult, Command};

fn main() {
    let exit_code = match run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        }
    };
    std::process::exit(exit_code);
}

fn run() -> CliResult<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = Command::from_args(&args)?;
    command.execute()?;
    println!();
    Ok(())
}
