# rustdex 🦀

[![Crates.io](https://img.shields.io/crates/v/rustdex?style=flat-square)](https://crates.io/crates/rustdex)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue?style=flat-square)](./LICENSE-MIT)
[![CI](https://github.com/pedrobrantes/rustdex/actions/workflows/rust.yml/badge.svg)](https://github.com/pedrobrantes/rustdex/actions)

A terminal-based Pokédex application written in Rust. This project serves as both a useful tool for Pokémon fans and a professional portfolio piece demonstrating best practices in Rust software engineering, including a modular architecture, comprehensive testing, and an automated CI/CD pipeline.

## Features

* Fetch and display Pokémon data directly from the terminal.
* Render Pokémon sprites as high-definition terminal graphics.
* Configurable image width for different terminal sizes.
* Modular and scalable architecture.
* (Planned) Subcommands for fetching data on Moves, Items, and more.
* (Planned) Fully interactive TUI (Text-based User Interface) mode.

## Installation

### With `cargo`
Once published, you can install `rustdex` directly from crates.io:
```bash
cargo install rustdex
```
## Usage

The primary way to use `rustdex` is through its subcommands.

### Fetching Pokémon Information

Use the `pokemon` subcommand to fetch details for a specific Pokémon.

```bash
rustdex --pokemon <POKEMON_NAME> [OPTIONS]

# Fetch Pikachu with default image width
rustdex -p pikachu

# Fetch Snorlax with a larger image
rustdex --pokemon snorlax --width 120
```
## Development

This project follows a professional Git workflow. All new features are developed on `feature/...` branches and merged into `develop` via Pull Requests.

To set up the development environment:
    Clone the repository.
    Install the pre-commit hooks: `pre-commit install`.
    Build the project: `cargo build`.
    Run tests: `cargo test`.

## License

This project is dual-licensed under the terms of both the [MIT License](./LICENSE-MIT) and the [Apache License 2.0](./LICENSE-APACHE).
