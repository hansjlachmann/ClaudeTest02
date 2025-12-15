# Hello World CLI - Rust

A simple command-line application written in Rust that greets the world.

## Features

- Prints "Hello, World!" by default
- Accepts an optional name argument for personalized greetings

## Building

```bash
cargo build --release
```

## Usage

Run without arguments:
```bash
./target/release/hello-world
# Output: Hello, World!
```

Run with a name:
```bash
./target/release/hello-world Rust
# Output: Hello, Rust!
```

## Requirements

- Rust 1.56.0 or higher (Rust 2021 edition)
