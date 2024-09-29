#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use alloc::{format};
use core::ffi::CStr;
#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();




use ckb_std::{syscalls};
use ckb_std::ckb_constants::Source;
use ckb_std::env::argv;
use ckb_std::syscalls::{current_cycles};

pub fn program_entry() -> i8 {
    if current_cycles() > 990000000 {
        return 0;
    }
    // spawn caller
    syscalls::exec(0, Source::CellDep, 0, 0, &[]);
    return 0;
}