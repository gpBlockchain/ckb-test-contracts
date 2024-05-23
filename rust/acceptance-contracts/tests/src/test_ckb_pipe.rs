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
fn test_read_write_max_data(){
    // xyl
    // 一次性读写比较大的buffer
    let name = "ckb_pipe";
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
        contract_bin,
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
       
    match context.verify_tx(&tx, cycle) {
    Ok(cycles) => {
        println!("test_success: consume cycles: {}", cycles);
    }
    Err(err) => {
        let error_message = format!("{:?}", err);
        assert!(error_message.contains("ValidationFailure") && error_message.contains("-1"), 
                "Unexpected error: {:?}", error_message);
        println!("Caught expected error: ValidationFailure with code -1");
        return;
    }
    }

}

#[test]
fn test_pipe_close_when_spawn_stop(){
    //After using ckb_exit or return 0, it's not possible to perform the ckb_close operation.
}
