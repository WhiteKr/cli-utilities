use crate::{CliError, CliResult};

#[derive(Debug)]
pub enum Command {
    Cat(Vec<String>),
    Echo(String),
    Ls(Option<String>),
    Help,
}

impl Command {
    pub fn from_args(args: &[String]) -> CliResult<Command> {
        if args.is_empty() {
            return Ok(Command::Help);
        }

        match args[0].as_str() {
            "cat" => {
                if args.len() < 2 {
                    return Err(CliError::InvalidArgument("No files specified".to_string()));
                }
                Ok(Command::Cat(args[1..].to_vec()))
            }
            "echo" => Ok(Command::Echo(args[1..].join(" "))),
            "ls" => Ok(Command::Ls(args.get(1).map(|s| s.to_string()))),
            "help" => Ok(Command::Help),
            _ => Err(CliError::InvalidArgument(format!(
                "Unknown command: {}",
                args[0]
            ))),
        }
    }

    pub fn execute(&self) -> CliResult<()> {
        match self {
            Command::Cat(files) => cat::execute(files),
            Command::Echo(text) => echo::execute(text),
            Command::Ls(path) => ls::execute(path.as_deref()),
            Command::Help => {
                print_help();
                Ok(())
            }
        }
    }
}

fn print_help() {
    println!("Available commands:");
    let commands = [
        ("cat <file>...", "Display contents of files"),
        ("echo <text>", "Display text"),
        ("ls [path]", "List directory contents"),
        ("help", "Show this help message"),
    ];

    let max_usage_len = commands
        .iter()
        .map(|(usage, _)| usage.len())
        .max()
        .unwrap_or(0);

    for (usage, description) in commands {
        println!(
            "  {:<width$}    {}",
            usage,
            description,
            width = max_usage_len
        );
    }
}

pub mod cat;
pub mod echo;
pub mod ls;
