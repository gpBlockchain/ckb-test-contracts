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

use ckb_std::{debug};
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::packed::Header;
use ckb_std::env::argv;
use ckb_std::error::SysError;
use ckb_std::syscalls::{current_memory, debug, load_block_extension, set_content};
use ckb_std::high_level::load_header;

pub fn program_entry() -> i8 {
    // ckb_load_extension();
    let mut data = [0u8; 100];

    debug!("load header");
    let header = match load_header(0, Source::CellDep) {
        Ok(header ) => {
            debug!("header:{:?}",header);

        }
        Err(err) => {
            return 2;
        }
    };
    debug!("header:{:?}",header);
    debug!("load extension");
    let result = load_block_extension(&mut data,0,0, Source::CellDep).unwrap();
    debug!("result:{:?}",result);
    return 0;
}