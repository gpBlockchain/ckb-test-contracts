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
use ckb_std::syscalls::{current_cycles, get_memory_limit, set_content, spawn};

///
/// int ckb_set_content(uint8_t* content, uint64_t* length);
/// invoke int ckb_spawn( uint64_t memory_limit,
///                    size_t index,
///                    size_t source,
///                    size_t bounds,
///                    int argc, char* argv[],
///                    int8_t* exit_code,
///                    uint8_t* content,
///                    uint64_t* content_length);
///
///     case1 : spawn(2)
///     result :
///         spawn return 5(Exceeded max content length.)

pub fn program_entry() -> i8 {
    let argvs = argv();
    if argvs.len() > 0 {
        let mut content1: [u8; 256 * 1024 + 1] = [1; 256 * 1024 + 1];
        content1[0] = argvs.len() as u8;
        let ret = set_content(&content1).unwrap();
        debug!("spawn succ, set_content result:{:?}",ret);
        return 0;
    }
    let mut exit_code: i8 = 0;
    // let mut out_of_max_length_content: [u8; 256*1024+1] = [0; 256*1024+1];
    let mut test_content: [u8; 10] = [2; 10];
    let content_length: u64 = test_content.len() as u64;
    let mut spawn_args = syscalls::SpawnArgs {
        memory_limit: 8,
        exit_code: &mut exit_code as *mut i8,
        content: test_content.as_mut_ptr(),
        content_length: &content_length as *const u64 as *mut u64,
    };
    let cstr1 = CStr::from_bytes_with_nul(b"arg0\0").unwrap();
    let cstrs = vec![cstr1];

    // let mut test_content: [u8; 10] = [2; 10];
    // spawn_args.content = test_content.as_mut_ptr();
    // spawn_args.content_length = 10 as * mut u64;
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    assert_eq!(result, 0);
    assert_eq!(exit_code, 0);
    // write failed
    assert_eq!(test_content, [2; 10]);
    debug!("context length:{:?},result:{:?}",test_content.len(),test_content);
    return 0;
}
