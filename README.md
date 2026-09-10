!!!NOTE!!!
For a newer version of the library, visit crates.io

# Quat RAM and CPU Library

A lightweight, high-performance Rust library for simulating a RAM and CPU architecture operating on **quats** (2-bit values representing 4 states: `0` to `3`).

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

use quat_ram::{CpuError, Quat, QuatCpu, QuatRam, StaticQuatRam};

fn main() -> Result<(), CpuError> {
    println!("=== 1. Quat Basics & Display ===");
    let q1 = Quat::Q3; // Binary 11 (Value 3)
    let q2 = Quat::Q1; // Binary 01 (Value 1)
    
    // Display trait allows direct printing
    println!("q1: {}, q2: {}", q1, q2); 

    // Bitwise operation on Quats
    let and_result = q1 & q2;
    println!("q1 AND q2 = {} (Value: {})", and_result, and_result.value());

    println!("\n=== 2. CPU Operations ===");
    let mut cpu = QuatCpu::new();
    
    // Load values into registers
    cpu.load(0, 1, 1)?; // Load 3 into reg[0]
    cpu.load(1, 0, 1)?; // Load 1 into reg[1]

    // Perform bitwise AND on registers
    cpu.and_regs(0, 1, 2)?; // reg[2] = reg[0] & reg[1]
    println!("CPU Reg[2] after AND: {}", cpu.registers[2]);

    println!("\n=== 3. Heap RAM (QuatRam) ===");
    let mut ram = QuatRam::new(16); // Dynamic RAM for 16 quats
    ram.write_quat(0, Quat::Q2)?;
    ram.write_quat(1, 3)?; // Accepts raw integer (u8) via TryInto

    println!("RAM[0]: {}", ram.read_quat(0)?);
    println!("RAM[1]: {}", ram.read_quat(1)?);

    println!("\n=== 4. Stack RAM (StaticQuatRam) ===");
    // Fixed-size RAM on stack (4 bytes holding up to 16 quats)
    let mut static_ram = StaticQuatRam::<4>::new(16)?;
    static_ram.write_quat(5, Quat::Q3)?;
    
    let value = static_ram.read_quat(5)?;
    println!("Static RAM Address 5: {}", value);

    Ok(())
}

```

## 🛠 Running Tests

To run the full suite of unit tests, use Cargo:

```bash
cargo test
```
