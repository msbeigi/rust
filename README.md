[![Rust](https://img.shields.io/badge/Rust-1.56.0-orange.svg)](https://www.rust-lang.org/)

# Rust Project

This project is a Rust application structured as a library and an executable, built using Cargo, the Rust package manager. It includes reusable functionality defined in the `lib.rs` file and an entry point executable in `main.rs`.

## Project Structure

- **src/:** Contains the source files.
  - **lib.rs:** Defines the library's functionality.
  - **main.rs:** Serves as the entry point for the executable application.

## Setup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# >1 proceed with defaults 
# To configure your current shell, run:
source $HOME/.cargo/env 

mkdir my_rust_project
cd my_rust_project

# Initialize a new Rust project
cargo init

cargo build

./target/debug/my_rust_project

cargo run
```

## make 
```
sudo apt update

sudo apt install make

make all
```