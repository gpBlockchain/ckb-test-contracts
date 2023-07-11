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
// spawn( [10],20)
// test_case1 = set_content([10])
// test_case2 = set_content([20])

use core::result::Result;

use alloc::{vec};
use core::ffi::{CStr};

use ckb_std::{debug, syscalls};
use ckb_std::ckb_constants::Source;
use ckb_std::env::argv;
use ckb_std::syscalls::{current_cycles, get_memory_limit, set_content, spawn, exec};


///
/// int ckb_set_content(uint8_t* content, uint64_t* length);
///
/// invoke int ckb_spawn( uint64_t memory_limit,
///                    size_t index,
///                    size_t source,
///                    size_t bounds,
///                    int argc, char* argv[],
///                    int8_t* exit_code,
///                    uint8_t* content,
///                    uint64_t* content_length);
///
///  case1 : spawn(content = [1:10], content_length = 20) -> {
///        set_content([2:10])
///     }
///  result
///      set_content return 10
///      spawn can query content == [2:10]
///
pub fn program_entry() -> i8 {
    let argvs = argv();
    // debug!("argvs length:{:?}:{:?}",argvs.len(),argvs);
    if argvs.len() == 1 {
        let memory_limit = get_memory_limit();
        debug!("memory_limit:{:?}",memory_limit);
        let mut content: [u8; 10] = [2; 10];
        let content_result = set_content(&content).unwrap();
        debug!("content_result:{:?}",content_result);
        assert_eq!(content_result, 10);
        return 0;
    }

    let mut exit_code: i8 = 0;
    let mut content: [u8; 10] = [1; 10];

    let content_length: u64 = (content.len() + 10) as u64;
    let mut spawn_args = syscalls::SpawnArgs {
        memory_limit: 8,
        exit_code: &mut exit_code as *mut i8,
        content: content.as_mut_ptr(),
        content_length: &content_length as *const u64 as *mut u64,
    };
    let cstr1 = CStr::from_bytes_with_nul(b"arg0\0").unwrap();
    //argv is empty
    let cstrs = vec![cstr1];
    spawn_args.memory_limit = 8;
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    assert_eq!(exit_code, 0);
    assert_eq!(content, [2; 10]);
    // debug!("result:{:?}",result);
    let cycles = current_cycles();
    debug!("cycle:{:?}",cycles);
    debug!("content:{:?}",content);
    return 0;
}
