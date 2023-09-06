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

use alloc::{format, vec};
use alloc::string::ToString;
use core::ffi::{CStr};
use core::ops::Index;

use ckb_std::{debug, syscalls};
use ckb_std::ckb_constants::Source;
use ckb_std::env::argv;
use ckb_std::syscalls::{current_cycles, current_memory, get_memory_limit, set_content, spawn};

///
/// test case :
/// invoke current_cycles
///
///     case1 : call current_cycles many times
///     result：
///        result must increase
///
///
pub fn program_entry() -> i8 {
    if current_memory() > 30 {
        let set_cycles = current_cycles();
        debug!("set u64:{}",set_cycles);
        set_content(&set_cycles.to_le_bytes()).expect("TODO: panic message");
        return 0;
    }

    let argvs = argv();
    let current_cycle1 = current_cycles();
    debug!("current cycle:{}, argv length:{} - 1",current_cycles(),argvs.len());
    if argvs.len() > 0 {
        debug!("current cycle:{}, argv length:{} -2 ",current_cycles(),argvs.len());
    }
    let mut exit_code: i8 = 0;
    let mut content: [u8; 8] = [0; 8];

    let content_length: u64 = content.len() as u64;
    let mut spawn_args = syscalls::SpawnArgs {
        memory_limit: 8,
        exit_code: &mut exit_code as *mut i8,
        content: content.as_mut_ptr(),
        content_length: &content_length as *const u64 as *mut u64,
    };


    let cstr1 = CStr::from_bytes_with_nul(b"arg\0").unwrap();
    let mut cstrs = vec![cstr1];
    for _i in 0..argvs.len() {
        cstrs.push(cstr1);
    }
    spawn_args.memory_limit = 7;
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    assert_eq!(result, 0);
    assert_eq!(exit_code, 0);
    debug!("arg length:{},content1:{:?}",argvs.len(),content);
    let restored_number = u64::from_le_bytes(content);
    debug!("Restored u64: {}", restored_number);
    assert!(restored_number > current_cycle1);
    let current_cycle2 = current_cycles();
    debug!("current cycle:{}, argv length:{} -3 ",current_cycle2,argvs.len());
    assert!(current_cycle2 > restored_number);
    content = [0; 8];
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    assert_eq!(result, 0);
    assert_eq!(exit_code, 0);
    debug!("arg length:{},content2:{:?}",argvs.len(),content);
    let restored_number2 = u64::from_le_bytes(content);
    debug!("Restored u64: {}", restored_number2);
    assert!(restored_number2 > current_cycle2);
    let current_cycle3 = current_cycles();
    debug!("current cycle:{}, argv length:{} -4 ",current_cycle3,argvs.len());
    assert!(current_cycle3 > restored_number2);
    let set_cycles = current_cycles();
    debug!("set u64:{}",set_cycles);
    set_content(&set_cycles.to_le_bytes()).expect("TODO: panic message");
    return 0;
}
