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
use ckb_std::syscalls::{current_cycles, debug, get_memory_limit, set_content, spawn};


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
///     case1 : content = 256K+1
///     result :
///         spawn return 5(Exceeded max content length.)
///
///     case2 : content = 256k
///     result :
///         spawn return 0(succ)
///
///     case3 : content = 256k , content_length = u64::MAX
///     result :
///        spawn return 5(Exceeded max content length.)
///
///     case4 : content = [1:10] , content_length = 10,
///     result :
///         spawn return 0, set_content succ, content = [256:10]
pub fn program_entry() -> i8 {
    let argvs = argv();
    if argvs.len() > 0 {
        let mut content1: [u8; 256*1024] = [1; 256*1024];
        content1[0] = argvs.len() as u8;
        let ret = set_content(&content1).unwrap();
        debug!("spawn succ, set_content result:{:?}",ret);
        let mut content1: [u8; 256*1024] = [u8::MAX; 256*1024];
        let ret = set_content(&content1).unwrap();
        debug!("spawn succ, set_content sed result:{:?}",ret);
        return 0;
    }
    let mut exit_code: i8 = 0;
    let mut out_of_max_length_content: [u8; 256*1024+1] = [0; 256*1024+1];

    let content_length: u64 = out_of_max_length_content.len() as u64;
    let mut spawn_args = syscalls::SpawnArgs {
        memory_limit: 8,
        exit_code: &mut exit_code as *mut i8,
        content: out_of_max_length_content.as_mut_ptr(),
        content_length: &content_length as *const u64 as *mut u64,
    };
    let cstr1 = CStr::from_bytes_with_nul(b"arg0\0").unwrap();
    let cstrs = vec![cstr1];
    spawn_args.memory_limit = 8;
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    debug!("content length : 256*1024+1 ,result:{:?}",result);
    assert_eq!(result, 5);

    let mut max_length_content: [u8; 256*1024] = [0; 256*1024];
    let max_content_length: u64 = max_length_content.len() as u64;
    spawn_args.content = max_length_content.as_mut_ptr();
    spawn_args.content_length = &max_content_length as *const u64 as *mut u64;
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    debug!("content 256*1024 ,result:{:?}",result);
    assert_eq!(max_length_content[0],u8::MAX);
    assert_eq!(result, 0);
    let mut test_content: [u8; 256] = [2; 256];
    spawn_args.content = test_content.as_mut_ptr();
    spawn_args.content_length = &mut u64::MAX;
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    debug!(" u64::MAX result:{:?}",result);
    debug!("exit result :{:?}",exit_code);
    debug!("context length:{:?},result:{:?}",test_content.len(),test_content);
    assert_eq!(result,5);

    let mut test_content: [u8; 10] = [2; 10];
    spawn_args.content = test_content.as_mut_ptr();
    let set_length = 10 as u64;
    spawn_args.content_length = &set_length as *const u64 as * mut u64;
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    debug!("result:{:?}",result);
    assert_eq!(result,0);
    assert_eq!(exit_code,0);
    // write failed
    assert_eq!(test_content,[u8::MAX; 10]);
    debug!("context length:{:?},result:{:?}",test_content.len(),test_content);
    return 0;
}
