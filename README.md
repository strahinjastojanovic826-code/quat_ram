# Quat RAM Library

A lightweight, high-performance Rust library for simulating a RAM architecture operating on **quats** (2-bit values representing 4 states: `0` to `3`).

Designed to be hardware-agnostic, crash-proof, and fully optimized using ultra-fast bitwise operations.

---

## 🚀 Features

* **Bitwise Efficiency:** Packs 4 quats into a single standard 8-bit byte (`u8`) with zero storage waste.
* **Crash-Proof Design:** Uses Rust's `Result<T, CpuError>` for all operations to ensure safe error handling instead of panicking.
* **Hardware Independent:** Pure software abstraction that compiles and runs smoothly on any architecture (x86, ARM, RISC-V) and memory type (DDR4, DDR5, etc.).
* **Open Source:** Licensed under the **GNU Affero General Public License v3.0 or later (AGPL-3.0-or-later)**.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
quat_ram = "0.1.1"
```

## 💡 Quick Start

Here is a quick example showing how to initialize the RAM, write quats, and read them back safely:

```rust

use quat_ram::{QuatCpu, QuatRam, CpuError};

fn main() -> Result<(), CpuError> {
    println!("--- Emulacija CPU-a ---");
    let mut cpu = QuatCpu::new();
    // Učitavamo bitove 1 i 1 (što daju kvat vrednost 3) u registar 0
    cpu.load(0, 1, 1)?;
    println!("Vrednost u registru 0: {}", cpu.registers[0]);

    println!("\n--- Emulacija RAM-a ---");
    let mut ram = QuatRam::new(8);
    // Upisujemo kvat vrednosti (0..3) na adrese u RAM-u
    ram.write_quat(0, 3)?;
    ram.write_quat(1, 1)?;

    // Čitamo nazad iz RAM-a
    let val = ram.read_quat(0)?;
    println!("Pročitana vrednost sa RAM adrese 0: {}", val);

    println!("\nSve radi savršeno!");
    Ok(())
}

```

## 🛠 Running Tests

To run the full suite of unit tests, use Cargo:

```bash
cargo test
```
