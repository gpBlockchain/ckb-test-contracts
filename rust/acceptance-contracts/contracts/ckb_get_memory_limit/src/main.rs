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
    let limit = get_memory_limit();
    if limit != 8 {
        // get_memory_limit == 8
        return 11;
    }
    return 0;
}
