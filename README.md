# cargo-maintenance-score

**Detect unmaintained and risky Rust dependencies with a smart health score**

A `cargo` subcommand that scans your project's dependencies and assigns each crate a **0–100 maintenance health score** based on:

- Days since the last crates.io update (major factor)
- Recent download popularity (helps offset older but still-used crates)
- Version
- Severity Status

It sorts the report **riskiest first** and highlights potentially unmaintained crates with clear warnings — perfect for avoiding security risks, bugs, or future breakage.

Built for the **Rust Africa Hackathon 2026 – Unmaintained Crates track**.

[![Crates.io](https://img.shields.io/crates/v/cargo-maintenance-score.svg)](https://crates.io/crates/cargo-maintenance-score)

## Example Output

<img width="1151" height="562" alt="Screenshot 2025-12-15 at 02 52 37" src="https://github.com/user-attachments/assets/49d02d59-b0bf-44fe-a23a-6a36a0fb2fa9" />
<img width="1139" height="761" alt="Screenshot 2025-12-15 at 03 08 48" src="https://github.com/user-attachments/assets/8dd2cc3a-9c1f-4f68-a207-c92070c3f542" />
<img width="741" height="360" alt="Screenshot 2025-12-15 at 03 09 28" src="https://github.com/user-attachments/assets/8fd42988-35b2-41f9-9325-c223139fe8a2" />


## Installation

```bash
cargo install cargo-maintenance-score
```

# Usage

Run it inside any Rust project directory (it reads your Cargo.lock):

```bash
cargo maintenance-score
```

Voila! It will fetch fresh data from crates.io and print a full colored report.

# Features

- Risk-based sorting (worst crates at the top)
- Clear risk levels: Low, Medium,  High
- Summary statistics
- Polite to crates.io
- No external dependencies beyond standard Rust tooling

# Why This Matters

Unmaintained crates are a real problem in the Rust ecosystem. 
Popular but abandoned packages (like ansi_term or arraystring) can introduce hidden risks. 
This tool helps you spot them early.Perfect for CI, pre-release checks, or just curiosity.

# Contributing:

Issues, PRs, and suggestions are very welcome!
Feel free to open an issue if you find a false positive/negative or want new scoring factors.






