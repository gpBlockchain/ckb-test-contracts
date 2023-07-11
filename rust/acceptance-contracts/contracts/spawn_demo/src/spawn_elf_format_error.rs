#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();


use core::result::Result;

use alloc::{vec};
use core::ffi::{CStr};

use ckb_std::{debug, syscalls};
use ckb_std::ckb_constants::Source;
use ckb_std::env::argv;
use ckb_std::syscalls::{current_cycles, get_memory_limit, set_content, spawn};
// use crate::{ckb_constants::*, error::SysError};


#[cfg(target_arch = "riscv64")]
use core::arch::asm;

#[cfg(target_arch = "riscv64")]
unsafe fn syscall(
    mut a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
) -> u64 {
    asm!(
    "ecall",
    inout("a0") a0,
    in("a1") a1,
    in("a2") a2,
    in("a3") a3,
    in("a4") a4,
    in("a5") a5,
    in("a6") a6,
    in("a7") a7
    );
    a0
}



///
/// test case :
/// invoke int ckb_spawn( uint64_t memory_limit,
///                    size_t index,
///                    size_t source,
///                    size_t bounds,
///                    int argc, char* argv[],
///                    int8_t* exit_code,
///                    uint8_t* content,
///                    uint64_t* content_length);
///
///    case1 : argc = 262143 ,argv = ["arg0"]
///    result :
///         spawn result = 4(Elf format error)
pub fn program_entry() -> i8 {
    let argvs = argv();
    if argvs.len() > 0 {
        debug!("argvs length:{:?}, :{:?}",argvs.len(),argvs);
        return 0;
    }
    let mut exit_code: i8 = 0;
    let mut content: [u8; 10] = [0; 10];

    let content_length: u64 = content.len() as u64;
    let mut spawn_args = syscalls::SpawnArgs {
        memory_limit: 8,
        exit_code: &mut exit_code as *mut i8,
        content: content.as_mut_ptr(),
        content_length: &content_length as *const u64 as *mut u64,
    };
    let cstr1 = CStr::from_bytes_with_nul(b"arg0\0").unwrap();
    let cstrs = vec![cstr1];
    debug!("set args:{:?}",cstrs);
    spawn_args.memory_limit = 8;
    let result = spawn_func(0, Source::CellDep, 0, 262143,cstrs.as_slice(), &spawn_args);
    debug!("exit:{:?}",exit_code);
    assert_eq!(result, 4);
    return 0;
}


pub fn spawn_func(index: usize, source: Source, bounds: usize,spawn_argc:u64, argv: &[&CStr], spgs: & syscalls::SpawnArgs) -> u64 {
    let argc = argv.len();
    let mut argv_ptr = alloc::vec![core::ptr::null(); argc + 1];
    for (idx, cstr) in argv.into_iter().enumerate() {
        argv_ptr[idx] = cstr.as_ptr();
    }
    unsafe {
        syscall(
            index as u64,
            source as u64,
            bounds as u64,
            spawn_argc,
            argv_ptr.as_ptr() as u64,
            spgs as *const syscalls::SpawnArgs as u64,
            0,
            2101,
        )
    }
}