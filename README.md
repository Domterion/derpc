<div align="center">
  <h1>💅 DeRPC</h1>
  A rotating Discord Rich Presence configured with TOML
  <br>
  <a href="https://rust-lang.org"><img src="https://img.shields.io/badge/rust-stable-brightgreen.svg" /></a>
  <a href="https://github.com/Domterion/derpc/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue" /></a>
  <a href="https://discord.gg/mMsy23j"><img src="https://img.shields.io/discord/640675047714848770.svg"/></a>
  <br />
</div>

- **Easy** to configure
- **Highly** configurable
- **Rotating** presences

# Using

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

# Configuration

A guide to the config can be found in `config.example.toml` then rename the config to `config.toml`. Make sure you read this and fill out the values.

# License
MIT
