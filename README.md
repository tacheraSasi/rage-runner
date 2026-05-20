# Rage Runner

A simple RUST based CLI tool that runs multiple shell commands concurrently using threads.

## Usage

```sh
rage-runner "command1" "command2" "command3"
```

## Example

```sh
rage-runner "echo hello" "sleep 2 && echo done" "ls -la"
```

All commands run in parallel. Results are printed as each command finishes.
