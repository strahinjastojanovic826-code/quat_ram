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
* **Unified Memory Abstraction:** Unified `Ram` trait enabling seamless interchangeability between heap-allocated and stack-allocated memory modules.
* **Execution Tracing:** Built-in CPU state tracing (`trace()`) for effortless step-by-step debugging and execution monitoring.
* **Bulk Memory Operations:** High-performance range manipulation methods (such as `memset`) for efficient buffer management.
* **Text Assembler:** Built-in parser (`assembler.rs`) for converting human-readable instructions (`NOP`, `ADD`, `SUB`, `JUMP`) directly into `Quat` machine code.
* **Immediate Addressing:** Direct constant loading into CPU registers via `load_immediate()`.
* **Multi-Core Thread Safety:** Safe multi-threaded CPU execution sharing `StaticQuatRam` utilizing `Arc<Mutex<T>>` architecture.
* **Instruction Decoding & Halt Control:** Separates decoding (`QuatInstruction`, `decode_next`) from execution (`execute`) over generic `Ram` implementations, with explicit `halted` state and `CpuError::Halted`.
* **Extended Quaternary Logic & Flags:** Adds 2-bit rotations (`rotl_quat`, `rotr_quat`), register comparison (`cmp_regs`), and unified `zero_flag` / `overflow_flag` updates across ALU operations.
* **Labels & Disassembly Output:** Supports label syntax and inline `;` comments in `Assembler::assemble_with_labels`, alongside string disassembly via `disassemble_to_string`.
* **Breakpoints & Step History (`CpuDebugger`):** Tracks recent `CpuTrace` snapshots and supports address breakpoints during stepped execution.
* **State Serialization & Restore:** Provides compact byte serialization and restoration (`to_bytes` / `from_bytes`) for `QuatRam`, `StaticQuatRam`, and `QuatCpu`.
* **Expanded FFI Status & Controls:** Introduces `FfiStatus` error codes and C-ABI functions for bulk RAM writes, CPU reset, and halt state inspection/control.

---

## Licensing

This project is dual-licensed:

* **Open Source (AGPLv3):** Free to use for open-source projects. If you use or modify this library in your software or network service, your source code must also be open-source under AGPLv3.
* **Commercial License:** Required if you want to use `quat_ram` in closed-source or proprietary commercial software without AGPLv3 restrictions.

For commercial licensing, contact: **strahinjastojanovic826@gmail.com**

---

**What's improved for users:**
* Demonstrates how the new `+` and `-` operators work directly on the `Quat` type.
* Added an example for the CPU `sub` method and how to retrieve `carry` and `borrow` flags.
* **Added Assembler Support:** Easily compile text instruction scripts into execution-ready `Quat` vectors.
* **Added Multi-Core Simulation:** Safely run concurrent CPU cores over shared memory buffers.

---

## 🚀 Benchmark and results:

| Test / Benchmark Block | Status | Iterations / Volume | Timing / Memory / Extra Metric | Throughput / Latency |
| :--- | :--- | :--- | :--- | :--- |
| Error Guard & Bounds Checking Overhead (`test_out_of_bounds_errors`) | `ok` | 1,000,000 checks | — | 104,743,848.92 checks/sec |
| CPU Add & Carry Evaluation (`test_cpu_add_and_carry`) | `ok` | 500,000 operations | Carry triggers: 500,000 | 25,593,120.57 ops/sec |
| CPU Subtraction & Borrow Flag (`test_cpu_sub_and_borrow`) | `ok` | 500,000 operations | Borrow triggers: 500,000 | 15,242,554.77 ops/sec |
| RAM Bit-Packing & Compression (`test_ram_packing`) | `ok` | 10 logical quats | Uncompressed: 10 bytes; Packed: ~34 bytes | Memory savings ratio: -240.0% |
| Static Stack-Allocated RAM (`test_static_ram`) | `ok` | 200,000 access cycles | Stack buffer size: 16 bytes | 25,665,045.49 operations/sec |
| CPU Execution Cycle & IPS (`test_cpu_ram_execution_cycle`) | `ok` | 100,000 full cycles (200,000 total steps) | Avg step latency: 207ns | 2,410,684.15 cycles/sec |
| Quat Rotations & Register Compare (`test_quat_rotations_compare_and_cpu_decode_halt`) | `ok` | 500,000 iterations | Total time: 51.5514ms | 9,699,057.64 pairs/sec |
| CPU Logic Instructions (`test_cpu_logic_ops`) | `ok` | 500,000 cycles | — | 18,307,038.32 inst/sec |
| FFI Interop Overhead (`test_ffi_interface`) | `ok` | 50,000 FFI cycles | — | 884,738.09 cycles/sec |
| Quat Conversion (`test_quat_conversion`) | `ok` | 1,000,000 iterations | Total time: 56.8714ms | 17,583,530.56 conversions/sec |
| Debugger + CPU Snapshot Roundtrip (`test_assembler_labels_disassembler_debugger_and_serialization`) | `ok` | 100,000 iterations | Total time: 85.9367ms | 1,163,647.20 cycles/sec |
| 1M Quats RAM Stress & Throughput (`test_stress_million_quats_ram`) | `ok` | 1,000,000 quats capacity | Memory allocated: 250,000 bytes (0.24 MB); Allocation speed: 43.4µs | Sequential write: 1,969,395,592 M quats/sec; Sampled read: 59,662,311.32 reads/sec |
| Quat Operator Overloading (Mod-4 Arithmetic) (`test_quat_add_sub_operators`) | `ok` | 1,000,000 arithmetic rounds | — | 36,576,076.87 calc/sec |
| RAM Bulk Read/Write (`test_ram_bulk_and_static_bounds`) | `ok` | 100,000 iterations | Total time: 77.1803ms | 1,295,667.42 batches/sec |
| Quat Bitwise Operations (`test_quat_bitwise_ops_and_display`) | `ok` | 2,000,000 batches (8,000,000 total ops) | — | 45,600,362.98 ops/sec |

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
quat_ram = "3.3.0"
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

    println!("\n=== 5. Assembler Demo ===");
let assembly_code = "
    NOP
    ADD
    SUB
    JUMP
";
let compiled_quats = quat_ram::assembler::Assembler::assemble(assembly_code).unwrap();
println!("Compiled {} instructions from text assembly.", compiled_quats.len());

    Ok(())
}
```

### Extended Features Example (Labels, Bulk RAM, Debugger & Snapshots)

```rust
use quat_ram::assembler::Assembler;
use quat_ram::cpu::{CpuDebugger, QuatCpu};
use quat_ram::error::CpuError;
use quat_ram::quat::Quat;
use quat_ram::ram::{QuatRam, Ram};

fn main() -> Result<(), CpuError> {
    let script = "
        START: NOP ; inline comment
        ADD
        SUB
    ";
    let program = Assembler.assemble_with_labels(script)?;

    let mut ram = QuatRam::new(16);
    ram.write_bulk(0, &program)?;

    let mut cpu = QuatCpu::new();
    cpu.load_immediate(0, Quat::Q2)?;
    cpu.load_immediate(1, Quat::Q1)?;

    let mut debugger = CpuDebugger::new(4);
    debugger.add_breakpoint(1);

    let executed = cpu.step_with_debugger(&ram, &mut debugger, false)?;
    println!("Executed step 0: {}, trace: {}", executed, cpu.trace());

    let snapshot = cpu.to_bytes();
    let restored_cpu = QuatCpu::from_bytes(&snapshot)?;
    println!("Restored PC: {}, halted: {}", restored_cpu.pc, restored_cpu.halted);

    Ok(())
}
```

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
```

### Extended C FFI Example (Bulk RAM Write, CPU Step, Halt & Reset)

```c
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stddef.h>

typedef enum {
    CQUAT_Q0 = 0,
    CQUAT_Q1 = 1,
    CQUAT_Q2 = 2,
    CQUAT_Q3 = 3
} CQuat;

void* quat_ram_new(size_t capacity_in_quats);
void quat_ram_free(void* ram);
int32_t quat_ram_write_bulk(void* ram, size_t start_addr, const CQuat* values, size_t len);
int32_t quat_ram_read(const void* ram, size_t quat_addr, CQuat* out_val);

void* quat_cpu_new(void);
void quat_cpu_free(void* cpu);
int32_t quat_cpu_step(void* cpu, const void* ram);
int32_t quat_cpu_get_register(const void* cpu, size_t reg_idx, CQuat* out_val);
int32_t quat_cpu_reset(void* cpu);
int32_t quat_cpu_halt(void* cpu);
int32_t quat_cpu_is_halted(const void* cpu, bool* out_halted);

int main(void) {
    void* ram = quat_ram_new(16);
    void* cpu = quat_cpu_new();

    CQuat program[2] = { CQUAT_Q1, CQUAT_Q0 };
    if (ram && cpu && quat_ram_write_bulk(ram, 0, program, 2) == 0) {
        quat_cpu_step(cpu, ram);

        CQuat reg0 = CQUAT_Q0;
        quat_cpu_get_register(cpu, 0, &reg0);
        printf("R0 after step: %d\n", reg0);

        bool halted = false;
        quat_cpu_halt(cpu);
        quat_cpu_is_halted(cpu, &halted);
        printf("CPU halted: %d\n", halted ? 1 : 0);

        quat_cpu_reset(cpu);
        quat_cpu_is_halted(cpu, &halted);
        printf("CPU halted after reset: %d\n", halted ? 1 : 0);
    }

    quat_cpu_free(cpu);
    quat_ram_free(ram);
    return 0;
}
```

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