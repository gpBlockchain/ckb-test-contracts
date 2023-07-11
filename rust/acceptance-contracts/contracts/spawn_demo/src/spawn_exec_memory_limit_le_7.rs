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
use ckb_std::syscalls::{current_cycles, get_memory_limit, set_content, spawn, exec};

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
///
pub fn program_entry() -> i8 {
    let argvs = argv();
    // debug!("argvs length:{:?}:{:?}",argvs.len(),argvs);
    if argvs.len() == 1 {
        let memory_limit = get_memory_limit();
        debug!("memory_limit:{:?}",memory_limit);
        let cstr1 = CStr::from_bytes_with_nul(b"arg0\0").unwrap();
        //argv is empty
        let cstrs = vec![cstr1, cstr1];
        let exec_ret = exec(0, Source::CellDep, 0, 0, cstrs.as_slice());
        debug!("exec result:{:?}",exec_ret);
        return 0;
    }
    if argvs.len() == 2 {
        let memory_limit = get_memory_limit();
        debug!("memory_limit:{:?}",memory_limit);
        return 0;
    }

    let mut exit_code: i8 = 0;
    let mut content: [u8; 10] = [1; 10];

    let content_length: u64 = content.len() as u64;
    let mut spawn_args = syscalls::SpawnArgs {
        memory_limit: 8,
        exit_code: &mut exit_code as *mut i8,
        content: content.as_mut_ptr(),
        content_length: &content_length as *const u64 as *mut u64,
    };
    let cstr1 = CStr::from_bytes_with_nul(b"arg0\0").unwrap();
    //argv is empty
    let cstrs = vec![cstr1];
    spawn_args.memory_limit = 7;
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    assert_eq!(exit_code, 0);
    // debug!("result:{:?}",result);
    let cycles = current_cycles();
    debug!("cycle:{:?}",cycles);
    return 0;
}
