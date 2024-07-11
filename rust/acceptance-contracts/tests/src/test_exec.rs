use std::time::Instant;
use ckb_testtool::ckb_types::core::ScriptHashType;
use crate::prelude::MAX_CYCLES;

#[test]
fn test_exec_in_exec_data2() {
    let time = Instant::now();
    crate::test_contract_type::test_contract_by_name_with_cycle_with_type("exec_with_exec", 1000_000_000, ScriptHashType::Data2);
    let cost_time = time.elapsed();
    println!("elapsed:{:?}", cost_time);
    assert!(cost_time.as_secs() < 20)
}

#[test]
fn test_exec_in_exec_data1() {
    let time = Instant::now();
    crate::test_contract_type::test_contract_by_name_with_cycle_with_type("exec_with_exec", 1000_000_000, ScriptHashType::Data1);
    let cost_time = time.elapsed();
    println!("elapsed:{:?}", cost_time);
    assert!(cost_time.as_secs() > 30)
}