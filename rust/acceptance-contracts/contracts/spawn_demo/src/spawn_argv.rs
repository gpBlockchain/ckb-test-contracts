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
/// test case :
/// invoke int ckb_spawn( uint64_t memory_limit,
///                    size_t index,
///                    size_t source,
///                    size_t bounds,
///                    int argc, char* argv[],
///                    int8_t* exit_code,
///                    uint8_t* content,
///                    uint64_t* content_length);
///
///  argv.length == 1
///
///     case1 :  argv.length == 1
///     result :
///         exit_code = 0
///         spawn return = 0
///
///     case2 : argv.length == 100
///     result :
///         exit_code = 0
///         spawn return = 0
///     case3 : argv.length = 8190
///     result :
///         exit_code = 0
///         spawn return = 0
pub fn program_entry() -> i8 {
    let argvs = argv();
    // debug!("argvs length:{:?}:{:?}",argvs.len(),argvs);
    if get_memory_limit() != 8 {
        return 0;
    }
    if argvs.len() != 0 {
        return 0;
    }
    let mut exit_code: i8 = 0;
    let mut content: [u8; 10] = [0; 10];

    let content_length: u64 = content.len() as u64;
    let mut spawn_args = syscalls::SpawnArgs {
        memory_limit: 8,
        exit_code: &mut exit_code as *mut i8,
        content: content.as_mut_ptr(),
        content_length: &content_length as *const u64 as *mut u64,
    };
    // let cstr1 = CStr::from_bytes_with_nul(b"arg0\0").unwrap();
    //argv is empty
    let cstrs = vec![];

    spawn_args.memory_limit = 7;
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    assert_eq!(exit_code, 0);
    debug!("result:{:?}",result);
    let mut cstr1 = CStr::from_bytes_with_nul(b"arg0\0").unwrap();
    let mut cstrs = vec![cstr1];
    for _ in 0..100 {
        cstrs.append(&mut vec![cstr1]);
    }
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    assert_eq!(exit_code, 0);
    assert_eq!(result, 0);
    let result = check_spawn_invoke_success(8190);
    assert_eq!(result,true);
    return 0;
}

fn find_spawn_argv(begin:usize,end:usize)->usize{
    for i in begin..end {
        if !check_spawn_invoke_success(i) {
            return i;
        }
    }
    return end;
}

fn find_spawn_argv_max_length(begin: usize, end: usize)->usize {


    let mut left = begin;
    let mut right = end;

    while left < right {
        let mid = left + (right - left) / 2;

        if check_spawn_invoke_success(mid) {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left - 1
}

fn check_spawn_invoke_success(argv_length: usize) -> bool {
    let mut exit_code: i8 = 0;
    let mut content: [u8; 10] = [0; 10];

    let content_length: u64 = content.len() as u64;
    let mut spawn_args = syscalls::SpawnArgs {
        memory_limit: 8,
        exit_code: &mut exit_code as *mut i8,
        content: content.as_mut_ptr(),
        content_length: &content_length as *const u64 as *mut u64,
    };
    let mut cstr1 = CStr::from_bytes_with_nul(b"a\0").unwrap();
    let mut cstrs = vec![cstr1];
    for _ in 0..argv_length {
        cstrs.append(&mut vec![cstr1]);
    }
    debug!("--- invoke argv.len()={:?}---",argv_length);
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    debug!("argv_length:{:?},result:{:?},exit:{:?}",argv_length,result,exit_code);
    assert_eq!(result, 0);
    if exit_code == 0 {
        return true;
    }
    return false;
}
