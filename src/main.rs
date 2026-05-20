use std::env;
use std::process::Command;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: rage-runner <command1> [command2] ...");
        return;
    }

    let commands: Vec<String> = args.iter().skip(1).cloned().collect();
    println!(
        "Rage Runner starting {} commands concurrently...\n",
        commands.len()
    );

    let mut handles = vec![];

    for cmd in commands {
        let handle = thread::spawn(move || {
            println!("▶️ Running: {}", cmd);

            let output = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .output()
                .expect("Failed to execute command");

            (cmd, output)
        });

        handles.push(handle);
    }

    // Waiting for all threads to finish
    let mut successes = 0;
    let mut failures = 0;

    for handle in handles {
        let (cmd, output) = handle.join().unwrap();

        println!(
            "\n[{}] {}",
            cmd,
            if output.status.success() {
                "✅ Success"
            } else {
                "❌ Failed"
            }
        );

        if !output.stdout.is_empty() {
            println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            eprintln!("Errors:\n{}", String::from_utf8_lossy(&output.stderr));
        }

        println!("{}", "─".repeat(50));

        if output.status.success() {
            successes += 1;
        } else {
            failures += 1;
        }
    }

    println!(
        "\nFinished! ✅ {} succeeded | ❌ {} failed",
        successes, failures
    );
}
