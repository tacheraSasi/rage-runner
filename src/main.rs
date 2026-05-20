use std::{env, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rage-runner <command1> [command2] ...");
        std::process::exit(1);
    }
    let command = &args[1];

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    println!("Output status: {}", output.status);

    if !output.stdout.is_empty() {
        println!("Standard Output:\n{}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.stderr.is_empty() {
        eprintln!("Standard Error:\n{}", String::from_utf8_lossy(&output.stderr));
    }
}
