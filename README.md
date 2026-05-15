# TP1 - Secure Log Analyzer in Rust

## Objective
This tool is a defensive security utility designed to analyze Linux authentication logs. 
It identifies failed SSH login attempts, counts them by IP address and username, and highlights suspicious activity without crashing on malformed data.

## Environment Requirements
- Docker & Docker Compose
- Rust Toolchain (Cargo 1.80+)

## Build Command
```bash
cargo build