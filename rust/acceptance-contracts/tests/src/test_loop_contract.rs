use super::*;
use ckb_testtool::context::Context;
use ckb_testtool::ckb_types::{
    bytes::Bytes,
    core::TransactionBuilder,
    packed::*,
    prelude::*,
};

const MAX_CYCLES: u64 = 1000_000_000;

// error numbers


#[test]
fn test_success() {
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("loop_contract");
    let out_point = context.deploy_cell(contract_bin);

    // prepare scripts
    let lock_script = context
        .build_script(&out_point, Bytes::from(vec![2]))
        .expect("script");

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(lock_script.clone()).pack())
            .build()
    ];

    let outputs_token:Vec<u128> = vec![5000];
    println!("outputs_token:{:?}",outputs_token);

    let outputs_data: Vec<_> = outputs_token
        .iter()
        .map(|token| Bytes::from(token.to_le_bytes().to_vec()))
        .collect();
    println!("outputs_data:{:?}",outputs_data);
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("test_success: consume cycles: {}", cycles);
}
