#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use alloc::{format, vec};
use alloc::string::ToString;
#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();



use ckb_std::{syscalls};
use ckb_std::ckb_constants::Source;
use ckb_std::env::argv;
use ckb_std::error::SysError;
use ckb_std::syscalls::{current_cycles};
use log::debug;

pub fn program_entry() -> i8 {
    let argvs = argv();
    print_current_cycle();
    let write_times = 10;

    if syscalls::process_id() > 0 {
        print_current_cycle();
        // spawn callee
        debug(format!("[spawn]argvs:{:?}", argvs));
        // assert_eq!(argvs, ["hello", "world"]);
        let mut std_fds: [u64; 2] = [0, 0];
        syscalls::inherited_file_descriptors(&mut std_fds);
        debug(format!("[spawn] inherited_file_descriptors fd:{:?}", std_fds));
        print_current_cycle();
        for _ in 0..write_times {
            match syscalls::write(std_fds[1], &[syscalls::process_id() as u8, syscalls::process_id() as u8, syscalls::process_id() as u8, syscalls::process_id() as u8]) {
                Ok(ok) => {
                    debug(format!("[spawn] write result:{:?}", ok));
                }
                Err(err) => {
                    debug(format!("[spawn] write faield result:{:?}", err));
                }
            };
            print_current_cycle();
            debug("---read data---".to_string());
            let mut read: [u8; 4] = [0, 0, 0, 0];
            match syscalls::read(std_fds[0], &mut read) {
                Ok(ok) => {
                    debug(format!("read fd:{:?},data:{:?},ret:{:?}", std_fds[0], read, ok));
                }
                Err(err) => {
                    debug(format!("read fd:{:?},data:{:?},err:{:?}", std_fds[0], read, err));
                }
            }
        }
        debug!("finished");
        return syscalls::process_id() as i8;
    }

    let mut root_process_write_fds = vec![];
    let mut root_process_read_fds = vec![];

    // spawn caller
    let pids = vec![0u64; 20];
    let spawn_times = 18u64;

    for i in 1..spawn_times {
        syscalls::debug(format!("current i:{:?}", i));
        let (root_read, spawn_write) = match syscalls::pipe() {
            Ok(ret) => {
                ret
            }
            Err(err) => {
                debug(format!("err:{:?}", err));
                (0u64, 0u64)
            }
        };
        let (spawn_read, root_write) = match syscalls::pipe() {
            Ok(ret) => {
                debug(format!("pipe:{:?}", ret));
                ret
            }
            Err(err) => {
                debug(format!("err:{:?}", err));
                (0u64, 0u64)
            }
        };
        debug(format!("root_read:{:?},spawn_write:{:?},spawn_read:{:?},root_write:{:?}", root_read, spawn_write, spawn_read, root_write));

        let pid: &u64 = pids.get(i as usize).unwrap();
        print_current_cycle();

        let mut inherited_fds = vec![spawn_read, spawn_write, 0];
        if spawn_read == spawn_write {
            inherited_fds = vec![0];
        }
        debug(format!("inherited_fds:{:?}", inherited_fds));
        match syscalls::spawn(0, Source::CellDep, 0, 0, &mut syscalls::SpawnArgs {
            argc: 0,
            argv: [].as_ptr(),
            process_id: *pid as *mut u64,
            inherited_fds: inherited_fds.as_slice().as_ptr(),
            // inherited_fds:[].as_ptr()
        }) {
            Ok(ok) => {
                debug(format!("invoke spawn:{:?},result:{:?}", i, ok));
                root_process_read_fds.push(root_read);
                root_process_write_fds.push(root_write);
                assert_eq!(ok, i);
            }
            Err(err) => {
                debug(format!("spawn:{:?},err:{:?}", i, err));
                assert_eq!(i, 17);
                assert_eq!(err, SysError::MaxVmsSpawned)
            }
        };
    }

    debug("write data ".to_string());
    debug(format!("root_process_read_fds:{:?}", root_process_read_fds));
    debug(format!("root_process_write_fds:{:?}", root_process_write_fds));
    for _ in 0..write_times {
        for j in 0..root_process_read_fds.len() {
            let read_fd = root_process_read_fds.get(j).unwrap();
            let write_fd = root_process_write_fds.get(j).unwrap();
            // read all
            let mut read: [u8; 4] = [0, 0, 0, 0];
            match syscalls::read(*read_fd, &mut read) {
                Ok(ok) => {
                    debug(format!("read fd:{:?},data:{:?},ret:{:?}", read_fd, read, ok));
                    let j1 = (j + 1) as u8;
                    assert_eq!(read, [j1; 4]);
                }
                Err(err) => {
                    debug(format!("read fd:{:?},data:{:?},err:{:?}", read_fd, read, err));
                }
            }
            match syscalls::write(*write_fd, &[0u8, 0u8, 0u8, 0u8]) {
                Ok(ok) => {
                    debug(format!("write fd:{:?},ret:{:?}", write_fd, ok));
                }
                Err(err) => {
                    debug(format!("write fd:{:?},ret:{:?}", write_fd, err));
                }
            }
            // write all
        }
    }
    for i in 1..spawn_times {
        match syscalls::wait(i) {
            Ok(ok) => {
                debug(format!("wait:{:?},result:{:?}", i, ok));
                assert_eq!(i, i);
            }
            Err(err) => {
                debug(format!("wait:{:?},result:{:?}", i, err));
                assert_eq!(i, 17);
            }
        }
    }
    for i in 0..root_process_read_fds.len() {
        let read_fd = root_process_read_fds.get(i).unwrap();
        let mut read: [u8; 4] = [0, 0, 0, 0];
        match syscalls::read(*read_fd, &mut read) {
            Ok(ok) => {
                debug(format!("read fd:{:?},data:{:?},ret:{:?}", read_fd, read, ok));
                assert!(false);
            }
            Err(err) => {
                debug(format!("read fd:{:?},data:{:?},err:{:?}", read_fd, read, err));
                assert_eq!(err, SysError::OtherEndClosed);
            }
        }
        let write_fd = root_process_write_fds.get(i).unwrap();

        match syscalls::write(*write_fd, &[0u8, 0u8, 0u8, 0u8]) {
            Ok(ok) => {
                debug(format!("write fd:{:?},ret:{:?}", write_fd, ok));
                assert!(false);
            }
            Err(err) => {
                debug(format!("write fd:{:?},ret:{:?}", write_fd, err));
                assert_eq!(err, SysError::OtherEndClosed);
            }
        }
    }

    return 0;
}

fn print_current_cycle() {
    let pid = syscalls::process_id();
    syscalls::debug(format!("id:{:?},cycle:{:?}", pid, current_cycles()))
}

fn debug(s: alloc::string::String) {
    syscalls::debug(format!("[{:?}]:{:?}", syscalls::process_id(), s));
}
