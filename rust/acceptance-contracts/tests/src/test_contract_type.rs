use std::time::Instant;
use super::*;
use ckb_testtool::context::Context;
use ckb_testtool::ckb_types::{
    bytes::Bytes,
    core::TransactionBuilder,
    packed::*,
    prelude::*,
};
use ckb_testtool::ckb_error::Error;
use ckb_testtool::ckb_types::core::{Cycle, ScriptHashType};

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
fn test_spawn_demo() {
    test_contract_by_name("spawn_demo");
}

#[test]
fn test_ckb_get_memory_limit() {
    test_contract_by_name("ckb_get_memory_limit")
}

#[test]
fn test_ckb_get_memory_limit_spawn() {
    test_contract_by_name("ckb_get_memory_limit_spawn")
}

#[test]
fn test_spawn_wrong_memory_limit() {
    test_contract_by_name("spawn_wrong_memory_limit")
}

#[test]
fn test_spawn_elf_format_error() {
    test_contract_by_name("spawn_elf_format_error")
}

#[test]
#[ignore]
fn test_spawn_exceeded_max_content_length() {
    test_contract_by_name("spawn_exceeded_max_content_length")
}

#[test]
fn test_set_content_spawn_length_exceeds_array_size_set_max_length() {
    test_contract_by_name("set_content_spawn_length_exceeds_array_size_set_max_length")
}

#[test]
fn test_spawn_exceeded_max_peak_memory() {
    test_contract_by_name("spawn_exceeded_max_peak_memory")
}


#[test]
#[should_panic(expected = "MemOutOfBound")]
fn test_spawn_argc_is_u64_max() {
    test_contract_by_name("spawn_argc_is_u64_max")
}


#[test]
fn test_spawn_argv() {
    test_contract_by_name("spawn_argv")
}

#[test]
fn test_spawn_index() {
    test_contract_by_name("spawn_index")
}

#[test]
#[ignore]
fn test_spawn_argc_not_eq() {
    test_contract_by_name("spawn_argc_not_eq")
}

#[test]
fn test_spawn_content() {
    test_contract_by_name("spawn_content")
}

#[test]
fn test_spawn_query() {
    test_contract_by_name("spawn_query")
}

#[test]
#[should_panic(expected = "ExceededMaximumCycles")]
fn test_spawn_times() {
    let time1 = Instant::now();
    test_contract_by_name_with_cycle("spawn_times", 1_000_000);
    let time = time1.elapsed();
    println!("time:{}", time.as_millis())
}


#[test]
fn test_spawn_recursive() {
    test_contract_by_name("spawn_recursive")
}

#[test]
#[should_panic(expected = "ExceededMaximumCycles")]
fn test_spawn_fib() {
    test_contract_by_name("spawn_fib")
}

#[test]
#[should_panic(expected = "MemOutOfBound")]
#[ignore]
fn test_spawn_out_of_memory() {
    test_contract_by_name("spawn_out_of_memory")
}

#[test]
#[should_panic(expected = "MemOutOfBound")]
fn test_spawn_exec_memory_limit_le_7() {
    test_contract_by_name("spawn_exec_memory_limit_le_7");
}

#[test]
fn test_spawn_exec_set_content() {
    test_contract_by_name("spawn_exec_set_content")
}

#[test]
fn test_spawn_exec_spawn() {
    test_contract_by_name("spawn_exec_spawn")
}

#[test]
fn test_set_content_without_spawn() {
    test_contract_by_name("set_content_without_spawn")
}

#[test]
fn test_set_content_many_times() {
    test_contract_by_name("set_content_many_times")
}

#[test]
fn test_set_content_exceed_length() {
    test_contract_by_name("set_content_exceed_length")
}

#[test]
fn test_set_content_insufficient_length() {
    test_contract_by_name("set_content_insufficient_length")
}

#[test]
fn test_set_content_nonzero_exit_no_rollback() {
    test_contract_by_name("set_content_nonzero_exit_no_rollback")
}

#[test]
fn test_set_content_data_propagation_to_parent_only() {
    test_contract_by_name("set_content_data_propagation_to_parent_only")
}

#[test]
fn test_set_content_spawn_length_exceeds_array_size_set_array_length() {
    test_contract_by_name("set_content_spawn_length_exceeds_array_size_set_array_length")
}

#[test]
fn test_set_content_spawn_length_exceeds_array_size_set_length() {
    test_contract_by_name("set_content_spawn_length_exceeds_array_size_set_length")
}

#[test]
fn test_set_content_spawn_length_less_than_array_size_set_array_length() {
    test_contract_by_name("set_content_spawn_length_less_than_array_size_set_array_length")
}

#[test]
fn test_set_content_spawn_length_less_than_array_size_set_length() {
    test_contract_by_name("set_content_spawn_length_less_than_array_size_set_length")
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
#[ignore]
fn test_spawn_current_cycles() {
    test_contract_by_name("spawn_current_cycles")
}

#[test]
fn test_spawn_current_memory() {
    test_contract_by_name("spawn_current_memory")
}

fn test_contract_by_name(name: &str) {
    test_contract_by_name_with_cycle(name, MAX_CYCLES);
}

fn test_contract_by_name_with_cycle(name: &str, cycle: u64) {
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary(name);
    let out_point = context.deploy_cell(contract_bin);

    // prepare headers
    let h1 = Header::new_builder()
        .raw(RawHeader::new_builder().number(1u64.pack()).build())
        .build()
        .into_view();
    context.insert_header(h1.clone());
    context.link_cell_with_block(out_point.clone(), h1.hash(), 0);
    // 不加
    context.block_extensions.insert(h1.hash(), Bytes::from_static(&[1, 2, 3]));

    // 加的话
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
    let input_out_point2 = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();

    let input2 = CellInput::new_builder()
        .previous_output(input_out_point2)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script)
            .build(),
    ];
    let inputs = vec![input, input2];

    let outputs_data = vec![Bytes::new(); 2];

    // build transaction
    let tx = TransactionBuilder::default()
        .inputs(inputs)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .header_dep(h1.hash())
        // .cell_deps(out_point.clone())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, cycle)
        .expect("pass verification");
    println!("test_success: consume cycles: {}", cycles);
}
