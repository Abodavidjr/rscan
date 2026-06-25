# rscan 🦀

A simple and fast multi-threaded TCP port scanner written in Rust.

## Features

- Multi-threaded scanning with Rayon
- Fast TCP connect scan
- Simple CLI interface
- Lightweight and easy to use
- Top 1000 common ports by default

## Installation

```bash
git clone https://github.com/Abodavidjr/rscan.git
cd rscan
cargo build --release

./target/release/rscan
Usage
    rscan <IP> [OPTIONS]
OPTIONS
    -p, --port     Scan specific ports (80,443,8080,9001)
    -a, --all      Scan ports 1-65535


🦀️🎯️🌐️🔥️ The Simple Port Scanner With Rust - rscan 🔥️🌐️🎯️🦀️

❗️ Usage:
    rscan <IP> [OPTIONS]
Output:
⚠️  Options:
    -p  --port     Scan Specific Ports <80,443,8080,9001>
    -a  --all      1-65535

🌐️  By default, scans the top 1000 common ports
