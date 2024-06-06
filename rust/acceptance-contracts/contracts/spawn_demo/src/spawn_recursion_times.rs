#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use alloc::{format};
#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();




use ckb_std::{syscalls};
use ckb_std::ckb_constants::Source;
use ckb_std::syscalls::{current_cycles};

pub fn program_entry() -> i8 {
    print_current_cycle();
    let vec_acc: [usize; 1024 * 488] = [0; 1024 * 488];
    // spawn caller
    for _ in 0..2 {
        for _ in 1..2 {
            // syscalls::debug(format!("current :{:?}", i));
            print_current_cycle();
            let inherited_fds: [u64; 1] = [0];
            let mut pid: u64 = 0;

            match syscalls::spawn(0, Source::CellDep, 0, 0, &mut syscalls::SpawnArgs {
                argc: 0,
                argv: [].as_ptr(),
                process_id: &mut pid as *mut u64,
                inherited_fds: inherited_fds.as_ptr(),
            }) {
                Ok(ok) => {
                    syscalls::debug(format!("ok:{:?}", ok));
                }
                Err(err) => {
                    syscalls::debug(format!("err:{:?}", err));
                }
            };
        }
    }
    let ret = vec_acc.get(0).unwrap();
    syscalls::debug(format!("ret:{:?}", ret));
    return 0;
}

fn print_current_cycle() {
    let pid = syscalls::process_id();
    syscalls::debug(format!("id:{:?},cycle:{:?}", pid, current_cycles()))
}
