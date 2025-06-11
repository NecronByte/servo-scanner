# 🧠 Servo-Scanner

**Servo-Scanner** is a terminal-based TCP port scanner with a grimdark tech-cult twist, built in Rust.

Designed for speed, aesthetics, and simplicity, it lets you scan single hosts across common or custom port ranges — all wrapped in NecronByte flair.

## 🚀 Features

- Scan ports on any IPv4 address or hostname
- Supports individual ports or ranges (e.g. `22,80,443`, `1-1024`)
- Styled, colored CLI output
- Instant feedback and sane timeouts

## 📦 Usage

```bash
cargo run -- <target> [--ports <portlist>]
```

**Examples:**

```bash
cargo run -- 127.0.0.1
cargo run -- scanme.nmap.org --ports 22,80,443
cargo run -- 192.168.1.10 --ports 1-100
```

## 🛠 Dependencies

- [clap](https://crates.io/crates/clap)
- [colored](https://crates.io/crates/colored)
- [tokio](https://crates.io/crates/tokio) *(optional for async version later)*

## 📜 License

MIT – use it, fork it, break it, improve it.

> _"Link established to sub-network... scanning for sentient sockets..."_