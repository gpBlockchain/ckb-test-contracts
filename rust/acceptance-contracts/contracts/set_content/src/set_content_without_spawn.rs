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
use ckb_std::env::argv;
use ckb_std::syscalls::{set_content};

///
/// int ckb_set_content(uint8_t* content, uint64_t* length);
///
///  case1 : set_content(xxx)
///  result
///       set_content == 0
pub fn program_entry() -> i8 {
    let mut content: [u8; 10] = [2; 10];
    let content_result = set_content(&content).unwrap();
    debug!("content_result:{:?}",content_result);
    assert_eq!(content_result, 0);
    return 0;
}
