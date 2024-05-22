use super::*;
use ckb_testtool::context::Context;
use ckb_testtool::ckb_types::{
    bytes::Bytes,
    core::TransactionBuilder,
    packed::*,
    prelude::*,
};
use ckb_testtool::ckb_error::Error;
use ckb_testtool::ckb_types::core::{ScriptHashType};
use crate::prelude::ContextExt;

const MAX_CYCLES: u64 = 1000_000_000;
// error numbers
const ERROR_EMPTY_ARGS: i8 = 0;

fn assert_script_error(err: Error, err_code: i8) {
    let error_string = err.to_string();
    assert!(
        error_string.contains(format!("error code {} ", err_code).as_str()),
        "error_string: {}, expected_error_code: {}",
        error_string,
        err_code
    );
}


#[test]
fn test_rfc49_atomic() {
    test_contract_by_name("rfc49_atomic")
}

#[test]
fn test_atomic_usize() {
    test_contract_by_name("atomic_usize")
}

#[test]
fn test_atomic_i8() {
    test_contract_by_name("atomic_i8")
}

#[test]
fn test_atomic_i16() {
    test_contract_by_name("atomic_i16")
}

#[test]
fn test_atomic_i32() {
    test_contract_by_name("atomic_i32")
}

#[test]
fn test_atomic_i64() {
    test_contract_by_name("atomic_i64")
}

#[test]
fn test_atomic_isize() {
    test_contract_by_name("atomic_isize")
}

#[test]
fn test_atomic_ptr() {
    test_contract_by_name("atomic_ptr")
}

#[test]
fn test_atomic_u8() {
    test_contract_by_name("atomic_u8")
}

#[test]
fn test_atomic_u16() {
    test_contract_by_name("atomic_u16")
}

#[test]
fn test_atomic_u32() {
    test_contract_by_name("atomic_u32")
}

#[test]
fn test_atomic_u64() {
    test_contract_by_name("atomic_u64")
}

#[test]
fn test_block_load_extension() {
    test_contract_by_name("load_block_extension")
}

#[test]
fn test_spawn_demo() {
    test_contract_by_name("spawn_demo")
}


fn test_contract_by_name(name: &str) {
    test_contract_by_name_with_cycle(name, MAX_CYCLES);
}

fn test_contract_by_name_with_cycle(name: &str, cycle: u64) {
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary(name);
    let out_point = context.deploy_cell(contract_bin.clone());

    // prepare headers
    let h1 = Header::new_builder()
        .raw(RawHeader::new_builder().number(1u64.pack()).build())
        .build()
        .into_view();
    context.insert_header(h1.clone());
    context.link_cell_with_block(out_point.clone(), h1.hash(), 0);

    context.block_extensions.insert(h1.hash(), Bytes::from_static(&[1, 2, 3]));

    // context.insert_extension(h1.hash(),Bytes::from_static(&[1,2,3]));

    // prepare scripts
    let lock_script = context
        .build_script_with_hash_type(&out_point, ScriptHashType::Data2, Bytes::from(vec![42]))
        .expect("script");

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );

    // build transaction
    let tx = TransactionBuilder::default()
        .inputs(vec![CellInput::new_builder()
            .previous_output(input_out_point)
            .build()])
        .outputs(vec![
            CellOutput::new_builder()
                .capacity(1000u64.pack())
                .lock(lock_script.clone())
                .build()
        ])
        .outputs_data(vec![Bytes::new(); 1].pack())
        .header_dep(h1.hash())
        // .cell_deps(out_point.clone())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .should_be_passed(&tx, cycle)
        .expect("pass verification");
    println!("test_success: consume cycles: {}", cycles);
}
