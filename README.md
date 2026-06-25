# rscan 🦀

A simple and fast multi-threaded TCP port scanner written in Rust.

## Features

* Multi-threaded scanning with Rayon
* Fast TCP connect scan
* Simple CLI interface
* Lightweight and easy to use
* Top 1000 common ports by default

## Installation

```bash
git clone https://github.com/Abodavidjr/rscan.git
cd rscan
cargo build --release
```

Run:

```bash
./target/release/rscan
```

## Usage

```bash
rscan <IP> [OPTIONS]
```

### Options

```text
-p, --port     Scan specific ports (80,443,8080,9001)
-a, --all      Scan ports 1-65535
```

## Example

```bash
./target/release/rscan 127.0.0.1 --help
```

Output:

```text
🦀️🎯️🌐️🔥️ The Simple Port Scanner With Rust - rscan 🔥️🌐️🎯️🦀️

❗️ Usage:
    rscan <IP> [OPTIONS]

⚠️  Options:
    -p  --port     Scan Specific Ports <80,443,8080,9001>
    -a  --all      1-65535

🌐️  By default, scans the top 1000 common ports
```

## Project Status

This project is still under development and is mainly a learning project for Rust, networking, and concurrency.

### Current Features

* Multi-threaded TCP port scanning
* CLI interface
* Top 1000 common ports scan
* Fast scanning with Rayon

### Planned Features

* Custom thread count
* Port ranges
* Better error handling
* Service detection
* Output formats (JSON, TXT)
* IPv6 support

## Looking for Contributors

This is an open-source learning project and contributions are welcome.

If you would like to help improve the project, feel free to:

* Open an Issue
* Suggest new features
* Report bugs
* Submit a Pull Request
* Improve documentation

Any help is appreciated.

## Why?

The goal of this project is to learn Rust, networking, concurrency, and open-source collaboration while building a useful tool.
