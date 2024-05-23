#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use alloc::format;
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
use ckb_std::syscalls::{current_cycles};
use alloc::vec;

const BUFFER_SIZE: usize = 512 * 1024; // 512KB buffer size

pub fn program_entry() -> i8 {
    let argvs = argv();
    print_current_cycle();
    if argvs.len() != 0 {
        print_current_cycle();
        // spawn callee
        let process_id = syscalls::process_id();
        syscalls::debug(format!("[spawn] process_id:{:?}", process_id));
        assert_eq!(process_id, 1);
        syscalls::debug(format!("[spawn]argvs:{:?}", argvs));
        let mut std_fds: [u64; 1] = [0];
        syscalls::inherited_file_descriptors(&mut std_fds);
        syscalls::debug(format!("[spawn] write fd:{:?}", std_fds[0]));
        print_current_cycle();

        // Create a 512KB buffer for writing
        syscalls::debug(format!("write buffer size:{:?}", BUFFER_SIZE));
        let write_buffer = vec![1u8; BUFFER_SIZE];
        let write_result = syscalls::write(std_fds[0], &write_buffer).unwrap();
        print_current_cycle();
        syscalls::debug(format!("[spawn] write result:{:?}", write_result));
        assert_eq!(write_result, BUFFER_SIZE);
        return 25i8;
    }

    // spawn caller
    let argc: u64 = 2;
    let argv = {
        let mut argv = alloc::vec![core::ptr::null(); argc as usize + 1];
        argv[0] = CStr::from_bytes_with_nul(b"hello\0").unwrap().as_ptr();
        argv[1] = CStr::from_bytes_with_nul(b"world\0").unwrap().as_ptr();
        argv
    };

    let mut son_fds: [u64; 2] = [0, 0];
    let (r0, w0) = syscalls::pipe().unwrap();
    son_fds[0] = w0;
    let mut pid: u64 = 0;
    let mut spgs = syscalls::SpawnArgs {
        argc: argc,
        argv: argv.as_ptr(),
        process_id: &mut pid as *mut u64,
        inherited_fds: son_fds.as_ptr(),
    };
    print_current_cycle();
    let spawn_result1 = syscalls::spawn(0, Source::CellDep, 0, 0, &mut spgs).unwrap();
    print_current_cycle();
    syscalls::debug(format!("spawn result:{:?}", spawn_result1));

    // Create a 512KB buffer for reading
    let mut read_buffer = vec![0u8; BUFFER_SIZE];
    print_current_cycle();
    let read_result = syscalls::read(r0, &mut read_buffer).unwrap();
    print_current_cycle();
    syscalls::debug(format!("read result:{:?}, data length: {:?}", read_result, read_result));
    assert_eq!(read_result, BUFFER_SIZE);
    assert_eq!(read_buffer[..4], [1, 1, 1, 1]); // Check the first 4 bytes
    print_current_cycle();

    let wait_result = syscalls::wait(spawn_result1).unwrap();
    print_current_cycle();
    syscalls::debug(format!("wait result:{:?}", wait_result));
    assert_eq!(wait_result, 25i8);
    syscalls::debug(format!("SpawnArgs.process_id:{:?}", pid));
    assert_eq!(pid, 1);
    return 0;
}

fn print_current_cycle() {
    let pid = syscalls::process_id();
    syscalls::debug(format!("id:{:?}, cycle:{:?}", pid, current_cycles()));
}
