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
use ckb_std::ckb_types::packed::{CellInput, CellOutput, Header};
use ckb_std::ckb_types::prelude::Entity;
use ckb_std::env::argv;
use ckb_std::error::SysError;
use ckb_std::high_level::{load_cell, load_cell_capacity, load_cell_data, load_header, load_input, load_script, load_script_hash, load_transaction, load_tx_hash, load_witness};
use ckb_std::syscalls::{current_cycles, load_cell_code, vm_version};

/// 1. invoke -> exec(1)
/// 2. exec(1) -> process_id
pub fn program_entry() -> i8 {
    get_block_opcode_2();
    return 0;
}


fn get_block_opcode_2() -> Vec<u8> {
    let mut vec1: Vec<u8> = vec![];
    let version = vm_version().unwrap();
    vec1.push(version as u8);
    debug!("version:{:?}",version);
    // assert_eq!(version, 1);
    let tx_hash = load_tx_hash().unwrap();
    vec1.extend(tx_hash.iter());
    debug!("tx_hash:{:?}",tx_hash);
    let script_hash = load_script_hash().unwrap();
    vec1.extend(script_hash.iter());
    debug!("script_hash:{:?}",script_hash);
    // load_cell_code(vec1,)
    match load_cell(999, Source::Output) {
        Ok(cell_output) => {
            assert!(false)
        }
        Err(_) => {}
    };
    match load_input(999, Source::Input) {
        Ok(input) => {
            assert!(false)
        }
        Err(_) => {}
    };
    let header = match load_header(999, Source::HeaderDep) {
        Ok(header) => {
            assert!(false)
        }
        Err(_) => {}
    };
    match load_witness(0, Source::Input) {
        Ok(_) => {
            assert!(false)
        }
        Err(_) => {}
    };
    match load_witness(999, Source::Input) {
        Ok(_) => {
            assert!(false)
        }
        Err(_) => {}
    };
    let transaction = load_transaction().unwrap();
    vec1.extend(transaction.as_slice());
    debug!("transaction:{:?}",transaction);
    debug!("transaction outputs length:{:?}",transaction.raw().outputs().len());
    match load_cell_capacity(999, Source::Input) {
        Ok(_) => {}
        Err(_) => {}
    };
    let script = load_script().unwrap();
    debug!("script:{:?}",script);
    let mut data = match load_cell_data(999, Source::Input) {
        Ok(_) => {}
        Err(_) => {}
    };
    debug!("vec1:{:?}",vec1);
    return vec1;
}
