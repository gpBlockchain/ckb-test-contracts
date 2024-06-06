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
    return 0;
}
