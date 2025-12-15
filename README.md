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

<img width="705" height="755" alt="Screenshot 2025-12-15 at 03 09 11" src="https://github.com/user-attachments/assets/1893df5a-41ba-4888-92bf-7f675794b2f0" />
<img width="1151" height="562" alt="Screenshot 2025-12-15 at 02 52 37" src="https://github.com/user-attachments/assets/ce3d1745-89a2-452e-910d-4a4adc4909b7" />
<img width="1440" height="900" alt="Screenshot 2025-12-15 at 03 09 17" src="https://github.com/user-attachments/assets/a2693744-576b-4ad4-8956-b6910930b690" />
<img width="1139" height="761" alt="Screenshot 2025-12-15 at 03 08 48" src="https://github.com/user-attachments/assets/85c3909a-24ae-45b8-89ef-4a8cec5b83ab" />



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






