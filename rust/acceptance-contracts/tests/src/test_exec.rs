use std::time::Instant;

#[test]
fn test_exec_in_exec() {
    let time = Instant::now();
    crate::test_contract_type::test_contract_by_name("exec_with_exec");
    println!("elapsed:{:?}", time.elapsed());
}
