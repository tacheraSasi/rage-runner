use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("No arguments provided. Please provide at least one command.");
        std::process::exit(1);
    }

    println!("you have provided {} commands", args.len() - 1);
    for i in 1..args.len() {
        println!("Command {}: {}", i, args[i]);
    }
}
