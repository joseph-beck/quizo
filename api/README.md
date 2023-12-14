# Rust backend

Uses both HTTP JSON API to communicate data as well as WebSockets to maintain session for game rooms etc.

## Requirements

- SQLite (SQLite Dev)
- Rust
- Diesel

## Usage

To build run

```sh
make build
```

To run run

```sh
make run
```

To test

```sh
make test
```

To clean

```sh
make clean
```

Please also make sure you have created a %name%.sqlite file and followed .env.example
