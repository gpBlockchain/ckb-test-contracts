#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use alloc::{format, vec};
use alloc::vec::Vec;
#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();



use core::ffi::CStr;

use ckb_std::{debug, syscalls};
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::prelude::Entity;
use ckb_std::env::argv;
use ckb_std::high_level::{load_cell, load_cell_capacity, load_cell_data, load_input, load_script, load_script_hash, load_transaction, load_tx_hash};
use ckb_std::syscalls::{current_cycles, vm_version};

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
        // assert_eq!(argvs, ["hello", "world"]);
        let mut std_fds: [u64; 1] = [0];
        syscalls::inherited_file_descriptors(&mut std_fds);
        syscalls::debug(format!("[spawn] write fd:{:?}", std_fds[0]));
        print_current_cycle();
        let opcode_result = get_block_opcode();
        let write_result = syscalls::write(std_fds[0], &opcode_result).unwrap();
        print_current_cycle();
        syscalls::debug(format!("[spawn] write result:{:?}", write_result));
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
    let opcode_data = get_block_opcode();
    let length = opcode_data.len();
    let mut read_data: Vec<u8> = vec![0; length];
    print_current_cycle();
    let read_result = syscalls::read(r0, &mut read_data).unwrap();
    print_current_cycle();
    syscalls::debug(format!("read result:{:?},data:{:?}", read_result, read_data));
    assert_eq!(read_data,opcode_data);
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
    syscalls::debug(format!("id:{:?},cycle:{:?}", pid, current_cycles()))
}


fn get_block_opcode() -> Vec<u8> {
    let mut vec1: Vec<u8> = vec![];
    let version = vm_version().unwrap();
    vec1.push(version as u8);
    debug!("version:{:?}",version);
    assert_eq!(version, 2);
    let tx_hash = load_tx_hash().unwrap();
    vec1.extend(tx_hash.iter());
    debug!("tx_hash:{:?}",tx_hash);
    let script_hash = load_script_hash().unwrap();
    vec1.extend(script_hash.iter());
    debug!("script_hash:{:?}",script_hash);
    let cell_output = load_cell(0, Source::Output).unwrap();
    vec1.extend(cell_output.as_slice());
    debug!("cell_output:{:?}",cell_output);
    let input = load_input(0, Source::Input).unwrap();
    vec1.extend(input.as_slice());
    debug!("input:{:?}",input);
    let transaction = load_transaction().unwrap();
    vec1.extend(transaction.as_slice());
    debug!("transaction:{:?}",transaction);
    debug!("transaction outputs length:{:?}",transaction.raw().outputs().len());
    let capacity = load_cell_capacity(0, Source::Input).unwrap();
    let script = load_script().unwrap();
    debug!("script:{:?}",script);
    debug!("capacity:{:?}",capacity);
    let mut data = load_cell_data(0, Source::Input).unwrap();
    debug!("data:{:?}",data.len());
    vec1.append(&mut data);
    debug!("vec1:{:?}",vec1);
    return vec1;
}

#[test]
fn test1234(){

}