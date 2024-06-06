use std::time::Instant;

#[test]
fn test_ckb_process_id_in_exec() {
    let time = Instant::now();
    crate::test_contract_type::test_contract_by_name("ckb_process_id_in_exec");
    println!("elapsed:{:?}", time.elapsed());
}

