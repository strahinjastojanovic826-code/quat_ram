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
use quat_ram::{QuatRam, CpuError};

fn main() -> Result<(), CpuError> {
    // Allocate RAM for 8 quats (which physically takes exactly 2 bytes)
    let mut ram = QuatRam::new(8);

    // Write quats (values 0..3) to specific quat addresses
    ram.write_quat(0, 3)?; // Quat 0 -> binary 11
    ram.write_quat(1, 1)?; // Quat 1 -> binary 01
    ram.write_quat(5, 2)?; // Quat 5 -> binary 10

    // Read back the quats
    assert_eq!(ram.read_quat(0)?, 3);
    assert_eq!(ram.read_quat(1)?, 1);
    assert_eq!(ram.read_quat(5)?, 2);

    // Unwritten addresses default to 0
    assert_eq!(ram.read_quat(2)?, 0);

    println!("Quat RAM simulation ran successfully!");
    Ok(())
}
```

## 🛠 Running Tests

To run the full suite of unit tests, use Cargo:

```bash
cargo test
```
