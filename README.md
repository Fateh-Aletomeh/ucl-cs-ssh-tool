# UCL CS SSH Tool

CLI tool that makes sshing into UCL CS lab machines less annoying :)

## Prerequisites

- **Rust & Cargo**: Rust **1.85.0** or newer, and `.cargo` is on your system's `PATH` (should be by default)
- **OpenSSH Client**: An `ssh` client installed and available in your system's `PATH`

## Setting up

By default, the name of the command will be `ucl-cs-ssh-tool`. If you want to change this, go to `cargo.toml` and update the `name` field.

To install the CLI, run:

```sh
cargo install --path .
```

This will install the program in the .cargo folder in your home directory.

## How to use

If you are unsure on how to use, simply run:

Linux

```sh
ucl-cs-ssh-tool -h
```

Windows

```sh
ucl-cs-ssh-tool.exe -h
```
