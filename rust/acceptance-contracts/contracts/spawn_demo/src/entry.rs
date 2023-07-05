// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

use alloc::{vec};
use core::ffi::{CStr};

use ckb_std::{debug, syscalls};
use ckb_std::ckb_constants::Source;
use ckb_std::env::argv;
use ckb_std::syscalls::{current_cycles, get_memory_limit, set_content, spawn};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    debug!("!!!before spawn, current_cycles:{:?}",current_cycles());

    // let tx = load_transaction().unwrap();
    // debug!("tx:{:?}",tx);
    // let tx = load_tx_hash().unwrap();
    // debug!("tx hash:{:?}",tx);
    // let version = vm_version().unwrap();
    // debug!("version:{:?}",version);
    let argvs = argv();
    debug!("argv:{:?}",argvs);
    // if argvs.len()>0{
    //     return Ok(());
    // }
    let limit = get_memory_limit();
    debug!("get_memory_limit:{:?}",limit);
    let mut init_memory_limit = 4;
    if argvs.len() > 5 {
        init_memory_limit = 5;
        let mut content1: [u8; 15] = [1; 15];
        content1[0] = argvs.len() as u8;
        let ret = set_content(&content1).unwrap();
        debug!("set result:{:?},content:{:?}",ret,content1);
        let ret: [u8; 1024] = [1; 1024];
        // let mut ret: Vec<u8> = Vec::with_capacity(1024 * 1024 * 1024 * 1024);
        return Err(Error::IndexOutOfBound);
    }
    let mut exit_code: i8 = 0;
    let mut content: [u8; 10] = [0; 10];

    let content_length: u64 = content.len() as u64;
    let spawn_args = syscalls::SpawnArgs {
        memory_limit: init_memory_limit,
        exit_code: &mut exit_code as *mut i8,
        content: content.as_mut_ptr(),
        content_length: &content_length as *const u64 as *mut u64,
    };
    let mut cstr1 = CStr::from_bytes_with_nul(b"arg0\0").unwrap();
    let mut cstrs = vec![cstr1];
    for _ in 0..argvs.len() {
        cstrs.append(&mut vec![cstr1]);
    }
    debug!("current exec spawn:{:?}",current_cycles());
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    debug!("exit:{:?}",exit_code);
    debug!("content:{:?}",content);
    debug!("spawn result:{:?}",result);
    debug!("after spawn,current_cycles:{:?}",current_cycles());
    return Ok(());
}
