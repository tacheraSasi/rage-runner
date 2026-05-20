use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    println!("you have provided {} commands", args.len() - 1);
    for i in 1..args.len() {
        println!("Command {}: {}", i, args[i]);
    }
}
