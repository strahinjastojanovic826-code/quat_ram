use std::sync::{Arc, Mutex};
use std::thread;
use crate::cpu::QuatCpu;
use crate::ram::StaticQuatRam;
use crate::quat::Quat;

pub struct MultiCoreSystem;

impl MultiCoreSystem {
    /// Demonstrates running two CPU cores sharing the same static RAM safely across threads.
    pub fn run_shared_ram_simulation() -> Result<(), Box<dyn std::error::Error>> {
        // Create a static RAM buffer shared safely via Arc and Mutex
        let ram = Arc::new(Mutex::new(StaticQuatRam::<64>::new(16).unwrap()));

        // Core 1 handle
        let ram_clone1 = Arc::clone(&ram);
        let handle1 = thread::spawn(move || {
            let mut cpu1 = QuatCpu::new();
            let _ = cpu1.load_immediate(0, Quat::Q2);
            
            // Lock RAM safely from thread 1
            let mut locked_ram = ram_clone1.lock().unwrap();
            let _ = locked_ram.write_quat(0, Quat::Q1); // Write instruction to RAM
        });

        // Core 2 handle
        let ram_clone2 = Arc::clone(&ram);
        let handle2 = thread::spawn(move || {
            let _cpu2 = QuatCpu::new();
            
            // Lock RAM safely from thread 2
            let locked_ram = ram_clone2.lock().unwrap();
            let _ = locked_ram.read_quat(0);
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        Ok(())
    }
}