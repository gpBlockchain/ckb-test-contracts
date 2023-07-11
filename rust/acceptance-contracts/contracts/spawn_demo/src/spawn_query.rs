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
use alloc::vec::Vec;
use core::ffi::{CStr};
use core::fmt::Debug;

use ckb_std::{debug, syscalls};
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::core::ScriptHashType::Data1;
use ckb_std::ckb_types::prelude::Entity;
use ckb_std::env::argv;
use ckb_std::high_level::{load_cell, load_cell_capacity, load_cell_data, load_header, load_header_epoch_start_block_number, load_input, load_input_since, load_script, load_script_hash, load_transaction, load_tx_hash, load_witness, look_for_dep_with_data_hash, spawn_cell};
use ckb_std::syscalls::{current_cycles, exec, get_memory_limit, load_cell_data_raw, set_content, spawn, vm_version};

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
///     query block  msg in spawn  == query block msg
///     case1 : spawn -> {
///             get_query_result()
///         }
///     result：
///         get_query_result() == spawn -> get_query_result()
///
///
pub fn program_entry() -> i8 {
    let argvs = argv();
    // assert_eq!(argvs.len(), 0);
    if get_memory_limit() != 8 {
        debug!("---- in spawn call----");
        let result = get_query_result();
        let slice: &[u8] = result.as_slice();
        let set_result = set_content(slice).unwrap();

        debug!("set_result:{:?}",set_result);
        // argv is empty
        return 0;
    }


    let query_result = get_query_result();
    let mut content: Vec<u8> = vec![];
    content.resize(query_result.len(), 0);
    let mut exit_code: i8 = 0;
    // let mut content: [u8; 100*1024] = [1; 100*1024];
    // content.
    let content_length: u64 = content.len() as u64;
    let mut spawn_args = syscalls::SpawnArgs {
        memory_limit: 8,
        exit_code: &mut exit_code as *mut i8,
        content: content.as_mut_ptr(),
        content_length: &content_length as *const u64 as *mut u64,
    };
    // let cstr1 = CStr::from_bytes_with_nul(b"arg0\0").unwrap();
    //argv is empty
    let cstrs = vec![];

    spawn_args.memory_limit = 7;
    let result = spawn(0, Source::CellDep, 0, cstrs.as_slice(), &spawn_args);
    assert_eq!(exit_code, 0);
    debug!("result:{:?}",result);
    debug!("content     :{:?}",content);
    debug!("query_result:{:?}",query_result);
    // assert_eq!(content, [1; 10]);
    assert_eq!(content, query_result);
    return 0;
}

fn get_query_result() -> Vec<u8> {
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
    let cell_output = load_cell(0, Source::CellDep).unwrap();
    vec1.extend(cell_output.as_slice());
    debug!("cell_output:{:?}",cell_output);
    let input = load_input(0, Source::Input).unwrap();
    vec1.extend(input.as_slice());
    debug!("input:{:?}",input);
    // let header = load_header(0, Source::HeaderDep).unwrap();
    // debug!("header:{:?}",header);
    // let witness = load_witness(0, Source::Input).unwrap();
    // debug!("witness:{:?}",witness);
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