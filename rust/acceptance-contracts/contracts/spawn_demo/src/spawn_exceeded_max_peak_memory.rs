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
///     case1 : Recursive calling until the VM's memory usage reaches the max peak memory.
///     result：
///         spawn return 7(Exceeded max peak memory)
pub fn program_entry() -> i8 {
    let argvs = argv();
    let mut exit_code: i8 = 0;
    let mut content: [u8; 1] = [0; 1];
    let content_length: u64 = content.len() as u64;
    let mut spawn_args = syscalls::SpawnArgs {
        memory_limit: 8,
        exit_code: &mut exit_code as *mut i8,
        content: content.as_mut_ptr(),
        content_length: &content_length as *const u64 as *mut u64,
    };
    let mut cstr1 = CStr::from_bytes_with_nul(b"arg0\0").unwrap();
    let mut cstrs = vec![cstr1];
    for _ in 0..argvs.len() {
        cstrs.append(&mut vec![cstr1]);
    }
    spawn_args.memory_limit = 8;
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    debug!("len:{:?},result:{:?}",argvs.len(),result);
    if argvs.len() == 7 {
        assert_eq!(result, 7)
    } else {
        assert_eq!(result, 0)
    }
    return 0;
}
