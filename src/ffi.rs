use crate::cpu::QuatCpu;
use crate::ram::QuatRam;
use crate::quat::Quat;
use crate::CpuError;

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

// SAFETY: `ram` is non-null and was allocated via `Box::into_raw(Box::new(QuatRam::new(...)))`
// in `quat_ram_new`; caller ensures it is freed at most once and not used afterwards.
#[unsafe(no_mangle)]
pub extern "C" fn quat_ram_free(ram: *mut QuatRam) {
    if !ram.is_null() {
        unsafe {
            let _ = Box::from_raw(ram);
        }
    }
}

// SAFETY: `ram` was checked for null; caller guarantees `ram` points to a valid, properly aligned
// `QuatRam` instance with no concurrent mutable or immutable aliases for the duration of this call.
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
    // SAFETY: `ram` was checked for null; caller guarantees `ram` points to a valid `QuatRam`
    // instance for the duration of the borrow.
    let ram = unsafe { &*ram };
    match ram.read_quat(quat_addr) {
        Ok(q) => {
            // SAFETY: `out_val` was checked for null; caller guarantees it is properly aligned and
            // valid for writing a `CQuat` value.
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

// SAFETY: `cpu` is non-null and originated from `Box::into_raw(Box::new(QuatCpu::new()))`
// in `quat_cpu_new`; caller ensures it is freed once and no aliases remain.
#[unsafe(no_mangle)]
pub extern "C" fn quat_cpu_free(cpu: *mut QuatCpu) {
    if !cpu.is_null() {
        unsafe {
            let _ = Box::from_raw(cpu);
        }
    }
}

// SAFETY: Both `cpu` and `ram` were checked for null; caller guarantees they point to valid,
// properly aligned instances and do not alias each other mutably during `step`.
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
    // SAFETY: `cpu` was checked for null; caller guarantees it points to a valid `QuatCpu`.
    let cpu = unsafe { &*cpu };
    // SAFETY: `out_val` was checked for null and `reg_idx < 4` was verified; `out_val` is valid
    // for a single `CQuat` write.
    unsafe { *out_val = CQuat::from(cpu.registers[reg_idx]) };
    0
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FfiStatus {
    Ok = 0,
    NullPointer = -1,
    InvalidAddress = -2,
    InvalidRegister = -3,
    InvalidValue = -4,
    CpuHalted = -5,
}

impl From<CpuError> for FfiStatus {
    fn from(err: CpuError) -> Self {
        match err {
            CpuError::InvalidAddress => FfiStatus::InvalidAddress,
            CpuError::InvalidRegister => FfiStatus::InvalidRegister,
            CpuError::InvalidValue => FfiStatus::InvalidValue,
            CpuError::Halted => FfiStatus::CpuHalted,
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_ram_write_bulk(
    ram: *mut QuatRam,
    start_addr: usize,
    values: *const CQuat,
    len: usize,
) -> i32 {
    if ram.is_null() || (len > 0 && values.is_null()) {
        return FfiStatus::NullPointer as i32;
    }
    // SAFETY: Pointers are checked for null; caller guarantees valid slice of `len` CQuat items.
    let ram = unsafe { &mut *ram };
    let slice = if len == 0 {
        &[]
    } else {
        // SAFETY: When `len > 0`, `values` was checked for null; caller guarantees `values` points to
        // `len` initialized `CQuat` elements valid for reads over the slice lifetime.
        unsafe { core::slice::from_raw_parts(values, len) }
    };
    let quats: Vec<Quat> = slice.iter().copied().map(Quat::from).collect();
    match crate::ram::Ram::write_bulk(ram, start_addr, &quats) {
        Ok(_) => FfiStatus::Ok as i32,
        Err(e) => FfiStatus::from(e) as i32,
    }
}

// SAFETY: `cpu` is non-null; caller guarantees a valid, exclusively accessible `QuatCpu`.
#[unsafe(no_mangle)]
pub extern "C" fn quat_cpu_reset(cpu: *mut QuatCpu) -> i32 {
    if cpu.is_null() {
        return FfiStatus::NullPointer as i32;
    }
    // SAFETY: Pointer checked for null and exclusively borrowed for reset.
    let cpu = unsafe { &mut *cpu };
    cpu.reset();
    FfiStatus::Ok as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn quat_cpu_halt(cpu: *mut QuatCpu) -> i32 {
    if cpu.is_null() {
        return FfiStatus::NullPointer as i32;
    }
    // SAFETY: Pointer checked for null and exclusively borrowed to set halted state.
    let cpu = unsafe { &mut *cpu };
    cpu.halt();
    FfiStatus::Ok as i32
}

// SAFETY: Both `cpu` and `out_halted` were checked for null; caller guarantees `cpu` is a valid
// `QuatCpu` and `out_halted` is properly aligned and valid for writing a `bool`.
#[unsafe(no_mangle)]
pub extern "C" fn quat_cpu_is_halted(cpu: *const QuatCpu, out_halted: *mut bool) -> i32 {
    if cpu.is_null() || out_halted.is_null() {
        return FfiStatus::NullPointer as i32;
    }
    // SAFETY: Both pointers are verified non-null and valid for single read/write.
    let cpu = unsafe { &*cpu };
    unsafe { *out_halted = cpu.halted };
    FfiStatus::Ok as i32
}