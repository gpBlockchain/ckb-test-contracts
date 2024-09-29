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
        let mut std_fds: [u64; 1] = [0];
        syscalls::inherited_fds(&mut std_fds);
        debug(format!("[spawn] inherited_fds fd:{:?}", std_fds));
        print_current_cycle();
        for _ in 0..write_times {
            match syscalls::write(std_fds[0], &[syscalls::process_id() as u8, syscalls::process_id() as u8, syscalls::process_id() as u8, syscalls::process_id() as u8]) {
                Ok(ok) => {
                    debug(format!("[spawn] write result:{:?}", ok));
                }
                Err(err) => {
                    debug(format!("[spawn] write faield result:{:?}", err));
                }
            };
            print_current_cycle();
        }
        debug!("finished");
        return syscalls::process_id() as i8;
    }

    let mut root_process_read_fds = vec![];

    // spawn caller
    let pids = vec![0u64; 20];
    let spawn_times = 17u64;

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

        debug(format!("root_read:{:?},spawn_write:{:?}", root_read, spawn_write));

        let pid: &u64 = pids.get(i as usize).unwrap();
        print_current_cycle();

        let inherited_fds = vec![spawn_write, 0];

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
                assert_eq!(ok, i);
            }
            Err(err) => {
                debug(format!("spawn:{:?},err:{:?}", i, err));
                assert_eq!(i, 17);
                assert_eq!(err, SysError::MaxVmsSpawned)
            }
        };
    }
    debug("stop one spawn ".to_string());
    let j = 0;
    for _ in 0..write_times {
        let read_fd = root_process_read_fds.get(j).unwrap();
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
    }
    syscalls::wait(1).unwrap();
    // create new spawn
    debug("create next 17 spawn".to_string());

    for i in 1..3 {
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

        debug(format!("root_read:{:?},spawn_write:{:?}", root_read, spawn_write));

        let pid: &u64 = pids.get(i as usize).unwrap();
        print_current_cycle();

        let inherited_fds = vec![spawn_write, 0];
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
                assert_eq!(i, 1)
            }
            Err(err) => {
                debug(format!("spawn:{:?},err:{:?}", i, err));
                assert_eq!(i, 2);
                assert_eq!(err, SysError::MaxVmsSpawned)
            }
        };
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
