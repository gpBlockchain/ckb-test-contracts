#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use alloc::{format};
#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();




use ckb_std::{syscalls};
use ckb_std::ckb_constants::Source;

pub fn program_entry() -> i8 {
    if syscalls::process_id() != 0 {
        return 0;
    }
    let inherited_fds: [u64; 1] = [0];
    let mut pid: u64 = 0;
    loop {
        if syscalls::current_cycles() > 990000000 {
            return 0;
        }
        syscalls::spawn(0, Source::CellDep, 0, 0, &mut syscalls::SpawnArgs {
            argc: 0,
            argv: [].as_ptr(),
            process_id: &mut pid as *mut u64,
            inherited_fds: inherited_fds.as_ptr(),
        }).unwrap();
    }
}
