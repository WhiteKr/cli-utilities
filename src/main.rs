use cli_utilities::commands;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!();
        return;
    }

    let command = &args[1];

    let result = match command.as_str() {
        "echo" => {
            let text = args[2..].join(" ");
            commands::echo::execute(&text)
        }
        "ls" => {
            let path = args.get(2).map(|s| s.as_str());
            commands::ls::execute(path)
        }
        _ => Err(format!("Unknown command: {}", command)),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    println!();
}
