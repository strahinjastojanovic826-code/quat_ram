use crate::cpu::QuatCpu;
use crate::ram::QuatRam;
use crate::quat::Quat;

/// C-compatible representation for Quat values
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CQuat {
    Q0 = 0,
    Q1 = 1,
    Q2 = 2,
    Q3 = 3,
}

impl From<Quat> for CQuat {
    fn from(q: Quat) -> Self {
        match q {
            Quat::Q0 => CQuat::Q0,
            Quat::Q1 => CQuat::Q1,
            Quat::Q2 => CQuat::Q2,
            Quat::Q3 => CQuat::Q3,
        }
    }
}

impl From<CQuat> for Quat {
    fn from(q: CQuat) -> Self {
        match q {
            CQuat::Q0 => Quat::Q0,
            CQuat::Q1 => Quat::Q1,
            CQuat::Q2 => Quat::Q2,
            CQuat::Q3 => Quat::Q3,
        }
    }
}

// ==========================================
// RAM FFI API
// ==========================================

#[unsafe(no_mangle)]
pub extern "C" fn quat_ram_new(capacity_in_quats: usize) -> *mut QuatRam {
    Box::into_raw(Box::new(QuatRam::new(capacity_in_quats)))
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_ram_free(ram: *mut QuatRam) {
    if !ram.is_null() {
        unsafe {
            let _ = Box::from_raw(ram);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_ram_write(ram: *mut QuatRam, quat_addr: usize, value: CQuat) -> i32 {
    if ram.is_null() {
        return -1;
    }
    let ram = unsafe { &mut *ram };
    match ram.write_quat(quat_addr, Quat::from(value)) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_ram_read(ram: *const QuatRam, quat_addr: usize, out_val: *mut CQuat) -> i32 {
    if ram.is_null() || out_val.is_null() {
        return -1;
    }
    let ram = unsafe { &*ram };
    match ram.read_quat(quat_addr) {
        Ok(q) => {
            unsafe { *out_val = CQuat::from(q) };
            0
        }
        Err(_) => -1,
    }
}

// ==========================================
// CPU FFI API
// ==========================================

#[unsafe(no_mangle)]
pub extern "C" fn quat_cpu_new() -> *mut QuatCpu {
    Box::into_raw(Box::new(QuatCpu::new()))
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_cpu_free(cpu: *mut QuatCpu) {
    if !cpu.is_null() {
        unsafe {
            let _ = Box::from_raw(cpu);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_cpu_step(cpu: *mut QuatCpu, ram: *const QuatRam) -> i32 {
    if cpu.is_null() || ram.is_null() {
        return -1;
    }
    let cpu = unsafe { &mut *cpu };
    let ram = unsafe { &*ram };
    match cpu.step(ram) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_cpu_get_register(cpu: *const QuatCpu, reg_idx: usize, out_val: *mut CQuat) -> i32 {
    if cpu.is_null() || out_val.is_null() || reg_idx >= 4 {
        return -1;
    }
    let cpu = unsafe { &*cpu };
    unsafe { *out_val = CQuat::from(cpu.registers[reg_idx]) };
    0
}