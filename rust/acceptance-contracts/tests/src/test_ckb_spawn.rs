use ckb_testtool::bytes::Bytes;
use ckb_testtool::ckb_types::core::{ScriptHashType, TransactionBuilder};
use ckb_testtool::ckb_types::packed::{CellInput, CellOutput, Header, RawHeader};
use ckb_testtool::ckb_types::prelude::{Builder, Entity, IntoHeaderView, Pack};
use ckb_testtool::context::Context;
use crate::Loader;
use crate::prelude::ContextExt;


/// build tx:
///     1. cellDep: spawn_source_is_input_cell(code)
///     2. input: code_hash:(spawn_source_is_input_cell) ,data: spawn_source_is_input_cell(code)
///     3. output: code_hash:(spawn_source_is_input_cell)
///     4. should pass
#[test]
fn test_source_is_input_cell() {
    let name = "spawn_source_is_input_cell";
    let cycle = 1000_000_000;
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary(name);

    // 1. cellDep: spawn_source_is_input_cell(code)
    let out_point = context.deploy_cell(contract_bin.clone());

    // prepare headers
    let h1 = Header::new_builder()
        .raw(RawHeader::new_builder().number(1u64.pack()).build())
        .build()
        .into_view();
    context.insert_header(h1.clone());
    context.link_cell_with_block(out_point.clone(), h1.hash(), 0);
    // 不加
    context.block_extensions.insert(h1.hash(), Bytes::from_static(&[1, 2, 3]));

    // 2. input: code_hash:(spawn_source_is_input_cell) ,data: spawn_source_is_input_cell(code)
    // prepare scripts
    let lock_script = context
        .build_script_with_hash_type(&out_point, ScriptHashType::Data2, Bytes::from(vec![42]))
        .expect("script");

    // prepare input  cells with contract bin
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        contract_bin,
    );

    // build transaction
    let tx = TransactionBuilder::default()
        .inputs(vec![CellInput::new_builder()
            .previous_output(input_out_point)
            .build()])
        //     3. output: code_hash:(spawn_source_is_input_cell)
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

    // 4. should pass
    // run
    let cycles = context
        .verify_tx(&tx, cycle)
        .expect("pass verification");
    println!("test_success: consume cycles: {}", cycles);
}

#[test]
fn test_index_is_out_of_range() {
    // 调用的index 不存在
    let name = "spawn_invaild_index";
    let cycle = 1000_000_000;
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
    // 不加
    context.block_extensions.insert(h1.hash(), Bytes::from_static(&[1, 2, 3]));

    // prepare scripts
    let lock_script = context
        .build_script_with_hash_type(&out_point, ScriptHashType::Data2, Bytes::from(vec![42]))
        .expect("script");

    // prepare input  cells with contract bin
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
        .witness(contract_bin.pack())
        // .cell_deps(out_point.clone())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, cycle)
        .expect("pass verification");
    println!("test_success: consume cycles: {}", cycles);
}


/// build tx:
///     1. cellDep: spawn_source_is_output_cell(code)
///     2. input: code_hash:(spawn_source_is_output_cell)
///     3. output: code_hash:(spawn_source_is_output_cell),data:spawn_source_is_output_cell(code)
///     4. should pass
#[test]
fn test_source_is_output_cell() {
    let name = "spawn_source_is_output_cell";
    let cycle = 1000_000_000;

    // 1. cellDep: spawn_source_is_output_cell(code)
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
    // 不加
    context.block_extensions.insert(h1.hash(), Bytes::from_static(&[1, 2, 3]));

    // prepare scripts
    let lock_script = context
        .build_script_with_hash_type(&out_point, ScriptHashType::Data2, Bytes::from(vec![42]))
        .expect("script");

    // 2. input: code_hash:(spawn_source_is_output_cell)
    // prepare input  cells with contract bin
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
        // 3. output: code_hash:(spawn_source_is_output_cell),data:spawn_source_is_output_cell(code)
        .outputs(vec![
            CellOutput::new_builder()
                .capacity(1000u64.pack())
                .lock(lock_script.clone())
                .build()
        ])
        .outputs_data(vec![contract_bin; 1].pack())
        .header_dep(h1.hash())
        // .cell_deps(out_point.clone())
        .build();
    let tx = context.complete_tx(tx);

    // 4. should pass
    // run
    let cycles = context
        .verify_tx(&tx, cycle)
        .expect("pass verification");
    println!("test_success: consume cycles: {}", cycles);
}

/// build tx:
///     1. cellDep: spawn_source_is_output_cell(code)
///     2. input: code_hash:(spawn_source_is_output_cell)
///     3. output: code_hash:(spawn_source_is_output_cell)
///     4. witness: [spawn_source_is_output_cell(code)]
///     5. should pass
#[test]
fn test_place_is_witness() {
    let name = "spawn_place_is_witness";
    let cycle = 1000_000_000;
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary(name);

    // 1. cellDep: spawn_source_is_output_cell(code)
    let out_point = context.deploy_cell(contract_bin.clone());

    // prepare headers
    let h1 = Header::new_builder()
        .raw(RawHeader::new_builder().number(1u64.pack()).build())
        .build()
        .into_view();
    context.insert_header(h1.clone());
    context.link_cell_with_block(out_point.clone(), h1.hash(), 0);
    // 不加
    context.block_extensions.insert(h1.hash(), Bytes::from_static(&[1, 2, 3]));

    // prepare scripts
    let lock_script = context
        .build_script_with_hash_type(&out_point, ScriptHashType::Data2, Bytes::from(vec![42]))
        .expect("script");

    // 1. cellDep: spawn_source_is_output_cell(code)
    // prepare input  cells with contract bin
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
        // 3. output: code_hash:(spawn_source_is_output_cell)
        .outputs(vec![
            CellOutput::new_builder()
                .capacity(1000u64.pack())
                .lock(lock_script.clone())
                .build()
        ])
        .outputs_data(vec![Bytes::new(); 1].pack())
        .header_dep(h1.hash())
        // 4. witness: [spawn_source_is_output_cell(code)]
        .witness(contract_bin.pack())
        // .cell_deps(out_point.clone())
        .build();
    let tx = context.complete_tx(tx);

    // 5. should pass
    // run
    let cycles = context
        .should_be_passed(&tx, cycle)
        .expect("pass verification");
    println!("test_success: consume cycles: {}", cycles);
}


/// build tx:
///     1. cellDep: spawn_source_is_output_cell(code)
///     2. input: code_hash:(spawn_source_is_output_cell)
///     3. output: code_hash:(spawn_source_is_output_cell)
///     4. should pass
#[test]
fn test_place_is_out_of_range() {
    // spawn_place_is_out_of_range
    let name = "spawn_place_is_out_of_range";
    let cycle = 1000_000_000;
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary(name);

    // 1. cellDep: spawn_source_is_output_cell(code)
    let out_point = context.deploy_cell(contract_bin.clone());

    // prepare headers
    let h1 = Header::new_builder()
        .raw(RawHeader::new_builder().number(1u64.pack()).build())
        .build()
        .into_view();
    context.insert_header(h1.clone());
    context.link_cell_with_block(out_point.clone(), h1.hash(), 0);
    // 不加
    context.block_extensions.insert(h1.hash(), Bytes::from_static(&[1, 2, 3]));

    // prepare scripts
    let lock_script = context
        .build_script_with_hash_type(&out_point, ScriptHashType::Data2, Bytes::from(vec![42]))
        .expect("script");

    // 2. input: code_hash:(spawn_source_is_output_cell)
    // prepare input  cells with contract bin
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
        // 3. output: code_hash:(spawn_source_is_output_cell)
        .outputs(vec![
            CellOutput::new_builder()
                .capacity(1000u64.pack())
                .lock(lock_script.clone())
                .build()
        ])
        .outputs_data(vec![Bytes::new(); 1].pack())
        .header_dep(h1.hash())
        .witness(contract_bin.pack())
        // .cell_deps(out_point.clone())
        .build();
    let tx = context.complete_tx(tx);

    // 4. should pass
    let cycles = context
        .verify_tx(&tx, cycle)
        .expect("pass verification");
    println!("test_success: consume cycles: {}", cycles);
}

/// ignore: debug code too big ,so load data will fail
#[test]
#[ignore]
fn test_bounds_is_not_0() {
    crate::test_contract_type::test_contract_by_name("spawn_bounds_is_not_0")
}

#[test]
fn test_bounds_is_out_of_range() {
    crate::test_contract_type::test_contract_by_name("spawn_bounds_is_out_of_range")
}

#[test]
#[should_panic(expected = "MemOutOfBound")]
fn test_spawn_args_argc_is_very_large() {
    // spawn调用 args 特别大
    let name = "spawn_argc_is_large";
    let cycle = 1000_000_000;
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
    // 不加
    context.block_extensions.insert(h1.hash(), Bytes::from_static(&[1, 2, 3]));

    // prepare scripts
    let lock_script = context
        .build_script_with_hash_type(&out_point, ScriptHashType::Data2, Bytes::from(vec![42]))
        .expect("script");

    // prepare input  cells with contract bin
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
        .witness(contract_bin.pack())
        // .cell_deps(out_point.clone())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, cycle)
        .expect("MemOutOfBound");
}

#[test]
fn test_spawn_inherited_fds_end_is_not_0() {
    // fds 不为0, return InvalidFd
    let name = "spawn_inherited_fds_end_is_not_0";
    let cycle = 1000_000_000;
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
    // 不加
    context.block_extensions.insert(h1.hash(), Bytes::from_static(&[1, 2, 3]));

    // prepare scripts
    let lock_script = context
        .build_script_with_hash_type(&out_point, ScriptHashType::Data2, Bytes::from(vec![42]))
        .expect("script");

    // prepare input  cells with contract bin
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
        .witness(contract_bin.pack())
        // .cell_deps(out_point.clone())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, cycle);
}

#[test]
fn test_spawn_inherited_fds_0_is_in_mid() {
    // fds 0在中间位置，预期应该返回invalid Fd，但实际返回成功
    let name = "spawn_inherited_fds_0_is_in_mid";
    let cycle = 1000_000_000;
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
    // 不加
    context.block_extensions.insert(h1.hash(), Bytes::from_static(&[1, 2, 3]));

    // prepare scripts
    let lock_script = context
        .build_script_with_hash_type(&out_point, ScriptHashType::Data2, Bytes::from(vec![42]))
        .expect("script");

    // prepare input  cells with contract bin
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
        .witness(contract_bin.pack())
        // .cell_deps(out_point.clone())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, cycle);
}

#[test]
fn test_spawn_inherited_fds_not_exist_fd() {
    // 不存在的fd
    let name = "spawn_inherited_fds_not_exist_fd";
    let cycle = 1000_000_000;
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
    // 不加
    context.block_extensions.insert(h1.hash(), Bytes::from_static(&[1, 2, 3]));

    // prepare scripts
    let lock_script = context
        .build_script_with_hash_type(&out_point, ScriptHashType::Data2, Bytes::from(vec![42]))
        .expect("script");

    // prepare input  cells with contract bin
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
        .witness(contract_bin.pack())
        // .cell_deps(out_point.clone())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, cycle);
}

#[test]
fn test_spawn_inherited_fds_contains_closed_fd() {
    // 关闭的fd
    let name = "spawn_inherited_fds_contains_closed_fd";
    let cycle = 1000_000_000;
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
    // 不加
    context.block_extensions.insert(h1.hash(), Bytes::from_static(&[1, 2, 3]));

    // prepare scripts
    let lock_script = context
        .build_script_with_hash_type(&out_point, ScriptHashType::Data2, Bytes::from(vec![42]))
        .expect("script");

    // prepare input  cells with contract bin
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
        .witness(contract_bin.pack())
        // .cell_deps(out_point.clone())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, cycle);
}

#[test]
fn test_spawn_inherited_fds_not_exist() {
    //check [0, 1] fds == 不传入fd值
    let name = "spawn_inherited_fds_not_exist";
    let cycle = 1000_000_000;
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
    // 不加
    context.block_extensions.insert(h1.hash(), Bytes::from_static(&[1, 2, 3]));

    // prepare scripts
    let lock_script = context
        .build_script_with_hash_type(&out_point, ScriptHashType::Data2, Bytes::from(vec![42]))
        .expect("script");

    // prepare input  cells with contract bin
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
        .witness(contract_bin.pack())
        // .cell_deps(out_point.clone())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, cycle);
}
#[test]
fn test_spawn_16_run_same_time() {
    crate::test_contract_type::test_contract_by_name("spawn_16_run_same_time")
}

#[test]
fn test_spawn_create_17_spawn() {
    crate::test_contract_type::test_contract_by_name("spawn_create_17_spawn")
}

#[test]
fn test_spawn_stop_16_spawn_create_17_spawn() {
    crate::test_contract_type::test_contract_by_name("spawn_stop_16_spawn_create_17_spawn")
}


#[test]
fn test_ckb_spawn_for(){
    crate::test_contract_type::test_contract_by_name("ckb_spawn_for")
}

#[test]
fn test_spawn_out_of_memory() {
    // xyl
    // spawn 里调用new buffer
}

#[test]
fn test_spawn_invoke_block_opcode() {
    crate::test_contract_type::test_contract_by_name("spawn_invoke_block_opcode");
}

#[test]
fn test_spawn_read_contract_is_bad() {
    // xyl
    // 合约不符合规范
}

#[test]
#[should_panic(expected = "ExceededMaximumCycles")]
fn test_spawn_loop_times() {
    crate::test_contract_type::test_contract_by_name("spawn_loop_times")
}

#[test]
#[should_panic(expected = "ExceededMaximumCycles")]
fn test_spawn_recursion_times() {
    crate::test_contract_type::test_contract_by_name("spawn_recursion_times")
}

#[test]
#[should_panic(expected = "ExceededMaximumCycles")]
fn test_cycle_inc_when_contains_recursion_and_loop_spawn() {
    crate::test_contract_type::test_contract_by_name("spawn_cycle_inc_when_contains_recursion_and_loop_spawn")
}


#[test]
fn test_spawn_can_stop_when_son_spawn_pause() {
    crate::test_contract_type::test_contract_by_name("spawn_can_stop_when_son_spawn_pause")
}