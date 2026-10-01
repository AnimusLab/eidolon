# Project Eidolon

> Stealth-hardened, cross-platform autonomous agent engineered in Rust for high-resilience, headless operation, and decentralized command tasking.

---

## Architecture & Core Features

Project Eidolon is built for secure, silent, and autonomous background execution. It combines rigorous systems-level hardening with a modular command-dispatch architecture:

* **Sarcophagus Environment Validation:** Active integrity checks that detect anomalous host environments or debugging hooks before executing critical payloads.
* **Decentralized Command Vector (`ghost_wallet`):** Headless polling mechanism that scans designated blockchain ledgers for instruction payloads encoded within transaction `OP_RETURN` outputs.
* **Cryptographic State Vault (`crypto_vault`):** Local state persistence secured via AES-256-GCM encryption, ensuring operational metadata is never left exposed in plaintext.
* **Camouflage & Jitter Masking (`camouflage`):** Randomized dormancy intervals that mimic standard background system telemetry to evade basic behavioral profiling.
* **Finite State Machine (FSM) Lifecycle:** Trait-based command pattern driving dynamic operational transitions between `Dormant`, `DiagnosticMode`, and `Lockdown`.

---

## Project Structure

eidolon/
├── Cargo.toml
├── src/
│   ├── main.rs          # Core orchestration loop, FSM switch, & signal handling
│   ├── commands.rs      # Trait-based command pattern & state transition handlers
│   ├── crypto_vault.rs  # AES-256-GCM local state encryption and vaulting
│   ├── ghost_wallet.rs  # Ledger polling and hex-encoded OP_RETURN parser
│   ├── camouflage.rs    # Randomized jitter delay generator
│   ├── sarcophagus.rs   # Platform environment integrity validation
│   ├── windows_impl.rs  # Windows-specific monitoring hooks
│   └── linux_impl.rs    # Linux target architecture hooks
└── eidolon_state.vault  # Encrypted local runtime state store

---

## Getting Started

### Prerequisites
* Rust toolchain (Edition 2021+)
* Cargo package manager

### Building for Release
Compile an optimized, release-ready binary:

    cargo build --release

### Running Tests
Execute the built-in parser and unit test harness offline:

    cargo test

### Execution
Run the compiled binary directly:

    cargo run --release

---

## License & Organization

Developed under the **AnimusLab** software research initiative. Maintained for autonomous systems research and cryptographic resilience engineering.