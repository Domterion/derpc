# DeRPC

![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
![Discord](https://img.shields.io/discord/640675047714848770.svg)

> A rotating Discord Rich Presence configured with TOML

- **Easy** to configure
- **Highly** configurable
- **Rotating** presences

### Using

Building:

To use you will need Rust and Git on your system.

```bash
# Clone the repo
$ git clone https://github.com/Domterion/derpc

# Cd into the cloned repo
$ cd derpc

# Build derpc for release
$ cargo build --release

# Fill out the config and move it
$ mv config.example.toml config.toml

# Run derpc, you must have a `config.toml` in this directory
$ ./target/release/derpc
```

Prebuilt:

Click the `Actions` tab then click the latest action. Download the respective zip and unzip it. Follow the configuration section below then run the `derpc` executable.

### Configuration

A guide to the config can be found in `config.example.toml` then rename the config to `config.toml`. Make sure you read this and fill out the values.
