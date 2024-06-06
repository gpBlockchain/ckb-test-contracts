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
use ckb_std::syscalls::{current_cycles, debug};

/// 1. invoke -> spawn(1) -> exec(1)
/// 2. exec(1) -> spawn(2)
/// 3. exec(0) -> spawn(1)
pub fn program_entry() -> i8 {
    if syscalls::current_cycles() > 990000000 {
        return 0;
    }
    syscalls::exec(0, Source::CellDep, 0, 0, &[]);
    return 0;
}

fn print_current_cycle() {
    let pid = syscalls::process_id();
    debug(format!("id:{:?},cycle:{:?}", pid, current_cycles()))
}
