// CStr::from_bytes_with_nul(b"arg1\0").unwrap(),

#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use alloc::{format};
use alloc::string::ToString;
#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();



use core::ffi::CStr;

use ckb_std::{syscalls};
use ckb_std::ckb_constants::Source;
use ckb_std::env::argv;
use ckb_std::syscalls::{current_cycles, debug};

/// 1. invoke -> exec(1)
/// 2. exec(1) -> process_id
pub fn program_entry() -> i8 {
    let argvs = argv();
    print_current_cycle();
    if argvs.len() != 0 {
        let id = syscalls::process_id();
        syscalls::debug(format!("process_id:{:?}",id));
        assert_eq!(id, 0);
        return 0;
    }
    syscalls::exec(0, Source::CellDep, 0, 0, &[CStr::from_bytes_with_nul(b"arg1\0").unwrap()]);
    return 0;
}

fn print_current_cycle() {
    let pid = syscalls::process_id();
    debug(format!("id:{:?},cycle:{:?}", pid, current_cycles()))
}
