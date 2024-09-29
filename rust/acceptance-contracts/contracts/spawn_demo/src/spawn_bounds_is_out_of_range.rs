#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use alloc::format;
use alloc::string::ToString;
#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();



use core::ffi::CStr;

use ckb_std::{syscalls};
use ckb_std::ckb_constants::Source;
use ckb_std::env::argv;
use ckb_std::error::SysError;
use ckb_std::syscalls::{current_cycles};

pub fn program_entry() -> i8 {
    syscalls::debug("---------------".to_string());
    let argvs = argv();
    print_current_cycle();
    if argvs.len() != 0 {
        syscalls::debug("---spawn ---".to_string());
        return 25i8;
    }

    // spawn caller
    let argc: u64 = 2;
    print_current_cycle();
    let argv = {
        let mut argv = alloc::vec![core::ptr::null(); argc as usize + 1];
        argv[0] = CStr::from_bytes_with_nul(b"hello\0").unwrap().as_ptr();
        argv[1] = CStr::from_bytes_with_nul(b"world\0").unwrap().as_ptr();
        argv
    };

    let mut son_fds: [u64; 1] = [ 0];
    print_current_cycle();
    let mut pid: u64 = 0;
    let mut spgs = syscalls::SpawnArgs {
        argc: argc,
        argv: argv.as_ptr(),
        process_id: &mut pid as *mut u64,
        inherited_fds: son_fds.as_ptr(),
    };
    print_current_cycle();
    let l = 99999999999u64;
    let h = 3 << 32;
    let bounds = h | l;
    syscalls::debug(format!("bounds :{:?}", bounds));
    match syscalls::spawn(0, Source::CellDep, 2, bounds as usize, &mut spgs) {
        Ok(_) => {
            assert!(false, "place is 2,should failed");
        }
        Err(err) => {
            syscalls::debug(format!("spawn_place_is_out_of_range err:{:?}", err));
            assert_eq!(err,SysError::IndexOutOfBound);
        }
    };
    return 0;
}

fn print_current_cycle() {
    let pid = syscalls::process_id();
    syscalls::debug(format!("id:{:?},cycle:{:?}", pid, current_cycles()))
}