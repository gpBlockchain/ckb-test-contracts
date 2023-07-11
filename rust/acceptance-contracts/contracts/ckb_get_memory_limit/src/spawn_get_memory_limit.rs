#![no_std]
#![cfg_attr(not(test), no_main)]

// define modules

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

/// program entry
pub fn program_entry() -> i8 {
    let argvs = argv();
    debug!("argv:{:?}",argvs);
    let limit = get_memory_limit();
    let mut init_memory_limit = 4;
    debug!("get_memory_limit:{:?}",limit);
    if argvs.len() >= 1 {
        if limit != 4 {
            // spawn memory_limit == 4
            return 12;
        }
    }

    if argvs.len() > 0 {
        return 0;
    }
    let mut exit_code: i8 = 0;
    let mut content: [u8; 10] = [0; 10];

    let content_length: u64 = content.len() as u64;
    let spawn_args = syscalls::SpawnArgs {
        memory_limit: init_memory_limit,
        exit_code: &mut exit_code as *mut i8,
        content: content.as_mut_ptr(),
        content_length: &content_length as *const u64 as *mut u64,
    };
    let cstr1 = CStr::from_bytes_with_nul(b"arg0\0").unwrap();
    let cstrs = vec![cstr1];
    debug!("current exec spawn:{:?}",current_cycles());
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    debug!("exit:{:?}",exit_code);
    debug!("content:{:?}",content);
    debug!("spawn result:{:?}",result);
    debug!("after spawn,current_cycles:{:?}",current_cycles());
    return 0;

}
