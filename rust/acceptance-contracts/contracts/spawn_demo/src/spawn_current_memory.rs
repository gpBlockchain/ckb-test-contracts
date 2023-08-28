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
use ckb_std::syscalls::{current_cycles, current_memory, get_memory_limit, set_content, spawn};

///
/// test case :
/// invoke current_cycles
///
///     case1 : index not exist
///     result：
///        spawn return 1
///
///
pub fn program_entry() -> i8 {
    let mut memory = current_memory();
    debug!("current memory:{}",memory);
    let argvs = argv();
    if argvs.len() > 0 {
        debug!("---- in spawn ----");
        let memory1 = current_memory();
        debug!("current memory:{}，argvs length:{}",memory1,argvs.len());
        assert_eq!(memory1 as usize, 8 + argvs.len() * 7);
        // return 0;
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
    let mut cstrs = vec![cstr1];
    for _i in 0..argvs.len() {
        cstrs.push(cstr1)
    }
    spawn_args.memory_limit = 7;
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    memory = current_memory();
    debug!("run succ .args length:{}current memory:{}",argvs.len(),memory);
    assert_eq!(memory as usize, 8 + argvs.len() * 7);
    debug!("result:{:?}",result);
    // assert_eq!(result, 0);
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    debug!("run succ .args length:{}current memory:{} -2",argvs.len(),memory);
    assert_eq!(memory as usize, 8 + argvs.len() * 7);
    debug!("result:{:?} -2",result);
    return 0;
}
