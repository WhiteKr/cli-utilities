use cli_utilities::commands;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!();
        return;
    }

    let text = args[1..].join(" ");

    if let Err(e) = commands::echo::execute(&text) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    println!();
}
