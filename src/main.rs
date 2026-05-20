use std::{env, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: rage-runner <command1> [command2] ...");
        std::process::exit(1);
    }

    println!("Running {} commands:", args.len() - 1);
    let commands = &args[1..];

    for (i, cmd) in commands.iter().enumerate() {
        println!("\nExecuting command {}: {}", i + 1, cmd);
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Failed to execute command");

        println!("Output status: {}", output.status);

        if !output.stdout.is_empty() {
            println!(
                "Standard Output:\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
        }

        if !output.stderr.is_empty() {
            eprintln!(
                "Standard Error:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
    println!("All commands finished!");
}
