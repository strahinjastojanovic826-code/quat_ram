# Quat RAM and CPU Library

A lightweight, high-performance Rust library for simulating a RAM and CPU architecture operating on **quats** (2-bit values representing 4 states: `0` to `3`).

Designed to be hardware-agnostic, crash-proof, and fully optimized using ultra-fast bitwise operations.

---

## 🚀 Features

* **Bitwise Efficiency:** Packs 4 quats into a single standard 8-bit byte (`u8`) with zero storage waste.
* **Cross-Language FFI:** Native C-compatible exports enabling seamless integration into C, C++, and C# applications.
* **Crash-Proof Design:** Uses Rust's `Result<T, CpuError>` for explicit error handling, eliminating runtime panics.
* **Static & Heap Allocation:** Supports both heap-allocated `QuatRam` and stack-allocated `StaticQuatRam` for embedded environments.
* **Arithmetic & Logic Operators:** Implements native Rust operator overloads (`+`, `-`, `&`, `|`, `^`, `!`) directly on the `Quat` type.

---

**What's improved for users:**
* Demonstrates how the new `+` and `-` operators work directly on the `Quat` type.
* Added an example for the CPU `sub` method and how to retrieve `carry` and `borrow` flags.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
quat_ram = "3.0.1"
```

## 💡 Quick Start

Here is a quick example showing how to initialize the RAM and CPU, write quats, and read them back safely:

```rust

use quat_ram::{CpuError, Quat, QuatCpu, QuatRam, StaticQuatRam};

fn main() -> Result<(), CpuError> {
    println!("=== 1. Quat Arithmetic & Logic ===");
    let q1 = Quat::Q3; // Binary 11 (Value 3)
    let q2 = Quat::Q1; // Binary 01 (Value 1)

    // Arithmetic operators (+ and -)
    println!("{} + {} = {}", q1, q2, q1 + q2); // 3 + 1 = 0 (mod 4)
    println!("{} - {} = {}", q1, q2, q1 - q2); // 3 - 1 = 2

    // Bitwise operation on Quats
    let and_result = q1 & q2;
    println!("q1 AND q2 = {} (Value: {})", and_result, and_result.value());

    println!("\n=== 2. CPU Operations & Flags ===");
    let mut cpu = QuatCpu::new();

    // Load values into registers
    cpu.load(0, 1, 1)?; // Load 3 into reg[0]
    cpu.load(1, 0, 1)?; // Load 1 into reg[1]

    // Perform CPU Addition & Subtraction
    let carry = cpu.add(0, 1, 2)?; // reg[2] = reg[0] + reg[1]
    println!("CPU Add Result reg[2]: {}, Carry: {}", cpu.registers[2], carry);

    let borrow = cpu.sub(0, 1, 3)?; // reg[3] = reg[0] - reg[1]
    println!("CPU Sub Result reg[3]: {}, Borrow: {}", cpu.registers[3], borrow);

    println!("\n=== 3. Heap RAM & Fetch-Execute Cycle ===");
    let mut ram = QuatRam::new(16); // Dynamic RAM for 16 quats

    // Write instructions and data into RAM
    ram.write_quat(0, Quat::Q1)?; // Program instruction at PC=0 (ADD)
    ram.write_quat(1, Quat::Q2)?; // Program instruction at PC=1 (SUB)

    // Execute instruction from RAM using Program Counter (PC)
    println!("Executing instruction at PC={}", cpu.pc);
    cpu.step(&ram)?;
    println!("Post-step PC: {}, Zero Flag: {}, Overflow Flag: {}", cpu.pc, cpu.zero_flag, cpu.overflow_flag);

    println!("\n=== 4. Stack RAM (StaticQuatRam) ===");
    // Fixed-size RAM on stack (4 bytes holding up to 16 quats)
    let mut static_ram = StaticQuatRam::<4>::new(16)?;
    static_ram.write_quat(5, Quat::Q3)?;

    let value = static_ram.read_quat(5)?;
    println!("Static RAM Address 5: {}, value: {}", value, value.value());

    Ok(())
}

FOR C C++

```c
#include <stdio.h>
#include <stdint.h>

typedef enum {
    CQUAT_Q0 = 0,
    CQUAT_Q1 = 1,
    CQUAT_Q2 = 2,
    CQUAT_Q3 = 3
} CQuat;

// Rust FFI bindings
void* quat_ram_new(size_t capacity_in_quats);
void quat_ram_free(void* ram);
int32_t quat_ram_write(void* ram, size_t addr, CQuat val);
int32_t quat_ram_read(const void* ram, size_t addr, CQuat* out_val);

int main() {
    void* ram = quat_ram_new(16);
    quat_ram_write(ram, 0, CQUAT_Q3);

    CQuat result;
    if (quat_ram_read(ram, 0, &result) == 0) {
        printf("Read quat from C: %d\n", result); // Prints 3
    }

    quat_ram_free(ram);
    return 0;
}

FOR C#

```c#
using System;
using System.Runtime.InteropServices;

public enum CQuat : int {
    Q0 = 0,
    Q1 = 1,
    Q2 = 2,
    Q3 = 3
}

class Program {
    const string LibName = "quat_ram";

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr quat_ram_new(UIntPtr capacityInQuats);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern void quat_ram_free(IntPtr ram);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern int quat_ram_write(IntPtr ram, UIntPtr addr, CQuat val);

    [DllImport(LibName, CallingConvention = CallingConvention.Cdecl)]
    public static extern int quat_ram_read(IntPtr ram, UIntPtr addr, out CQuat outVal);

    static void Main() {
        IntPtr ram = quat_ram_new((UIntPtr)16);
        quat_ram_write(ram, (UIntPtr)0, CQuat.Q2);

        if (quat_ram_read(ram, (UIntPtr)0, out CQuat value) == 0) {
            Console.WriteLine($"Read quat from C#: {value}"); // Q2
        }

        quat_ram_free(ram);
    }
}

```

## 🛠 Running Tests

To run the full suite of unit tests, use Cargo:

```bash
cargo test
```