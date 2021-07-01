# DeRPC

![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)

> A rotating Discord Rich Presence configured with TOML

- **Easy** to configure
- **Highly** configurable
- **Rotating** presences

### Using

To use you will need Rust and Git on your system.
_or download the release_

```bash
# Clone the repo
$ git clone https://github.com/Domterion/derpc

# Cd into the cloned repo
$ cd derpc

# Build derpc for release
$ cargo build --release

# Run derpc, you must have a `config.toml` in this directory
$ ./target/release/derpc
```

### Configuration

A guide to the config can be found in `config.example.toml` then rename the config to `config.toml`. Make sure you read this and fill out the values.
