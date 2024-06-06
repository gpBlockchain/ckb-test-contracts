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
use ckb_std::syscalls::SysError;


const BUFFER_SIZE: usize = 256 * 1024; // 256KB buffer size

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

        // Create a 256KB buffer for writing
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

    let mut child_fds: [u64; 2] = [0, 0];
    // ckb_pipe generate a pair r0,w0 fd
    let (r0, w0) = syscalls::pipe().unwrap();
    child_fds[0] = w0;
    let mut pid: u64 = 0;
    let mut spgs = syscalls::SpawnArgs {
        argc: argc,
        argv: argv.as_ptr(),
        process_id: &mut pid as *mut u64,
        inherited_fds: child_fds.as_ptr(),
    };
    print_current_cycle();
    let spawn_result1 = syscalls::spawn(0, Source::CellDep, 0, 0, &mut spgs).unwrap();
    print_current_cycle();
    syscalls::debug(format!("spawn result:{:?}", spawn_result1));

    // Create a 256KB buffer for reading
    let mut read_buffer = vec![0u8; BUFFER_SIZE];
    print_current_cycle();
    let read_result = syscalls::read(r0, &mut read_buffer).unwrap();
    print_current_cycle();
    syscalls::debug(format!("read result:{:?}, data length: {:?}", read_result, read_result));
    assert_eq!(read_result, BUFFER_SIZE);
    assert_eq!(read_buffer[..4], [1, 1, 1, 1]); // Check the first 4 bytes
    print_current_cycle();
    //ckb_wait until child process exit
    let wait_result = syscalls::wait(spawn_result1).unwrap();
    print_current_cycle();
    syscalls::debug(format!("wait result:{:?}", wait_result));
    //if child process exit code = 25i8, child process exit success, w0 r0 all exit
    assert_eq!(wait_result, 25i8);
    //read
    // let close_fd_r0 = syscalls::ckb_close(r0).unwrap();
    // syscalls::debug(format!("close r0 result:{:?}", close_fd_r0));

    match syscalls::read(r0, &mut read_buffer) {
        Ok(read_result) => {
            syscalls::debug(format!("Read {} bytes successfully.", read_result));
            assert_eq!(read_buffer[..4], [1, 1, 1, 1]); // Check the first 4 bytes
        },
        //https://github.com/nervosnetwork/ckb-std/blob/56c7541b38814089c46830d7ccb53450febfe9ac/src/error.rs#L21
        Err(err) => {
            let error_message = format!("Failed to read: {:?}", err);
            syscalls::debug(format!("Caught error: {:?}", error_message));
            assert_eq!(error_message, "Failed to read: OtherEndClosed");
        }
    }
    syscalls::debug(format!("read result:{:?}", read_result));
    print_current_cycle();
    match syscalls::spawn(0, Source::CellDep, 0, 0, &mut spgs) {
        Ok(spawn_result) => {
            syscalls::debug(format!("[result]:{:?}", spawn_result));
        }
        Err(err) => {
            syscalls::debug(format!("err:{:?}", err));
            assert_eq!(err,SysError::InvalidFd);
        }
    };
    syscalls::debug(format!("SpawnArgs.process_id:{:?}", pid));
    assert_eq!(pid, 1);
    return 0;
}

fn print_current_cycle() {
    let pid = syscalls::process_id();
    syscalls::debug(format!("id:{:?}, cycle:{:?}", pid, current_cycles()));
}
