#[test]
fn test_source_is_input_cell() {
    // gp
    // spawn 调用 input cell
}

#[test]
fn test_index_is_out_of_range(){
    // xyl
    // 调用的index 不存在
}

#[test]
fn test_source_is_output_cell() {
    // gp
    // spawn 调用 output cell
}

#[test]
fn test_place_is_witness() {
    // gp
    // spawn 调用 witness 合约

}

#[test]
fn test_bounds_is_not_0() {
    // gp
    // spawn 调用 bounds
}

#[test]
fn test_bounds_is_out_of_range() {
    // gp
    // bounds 超出边界
}

#[test]
fn test_spawn_args_argc_is_very_large() {
    //xyl
    // spawn调用 args 特别大
}

#[test]
fn test_spawn_inherited_fds_end_is_not_0() {
    // xyl
    // fds 不为0
}

#[test]
fn test_spawn_inherited_fds_0_is_in_mid() {
    // xyl
    // fds 0在中间位置
}

#[test]
fn test_spawn_inherited_fds_exist_repeated_fd() {
    // xyl
    // 存在重复的fd
    // 检查重复的fd是否都可以使用
    // 如果都能使用检查将其中一个fd传递出去，这时候2个fd会怎么样
}

#[test]
fn test_spawn_inherited_fds_not_exist_fd() {
    // xyl
    // 不存在的fd
}

#[test]
fn test_spawn_inherited_fds_contains_closed_fd() {
    // xyl
    // 关闭的fd
}

#[test]
fn test_spawn_16_run_same_time() {
    // gp
    //
}

#[test]
fn test_spawn_create_17_spawn() {
    // gp
}

#[test]
fn test_spawn_stop_16_spawn_create_17_spawn() {
    // gp
}


#[test]
fn test_spawn_out_of_memory() {
    // xyl
    // spawn 里调用new buffer
}

#[test]
fn test_spawn_invoke_block_opcode() {
    // gp
    // 调用一系列opcode
}

#[test]
fn test_spawn_read_contract_is_bad() {
    // xyl
    // 合约不符合规范
}

#[test]
fn test_spawn_loop_times() {
    // gp
    // spawn 调用次数
}

#[test]
fn test_spawn_recursion_times() {
    //gp
    // 递归上限
}

#[test]
fn test_cycle_inc_when_contains_recursion_and_loop_spawn(){
    // gp

}


#[test]
fn test_spawn_can_stop_when_son_spawn_pause(){
    // gp
    // 子进程停了，子进程生成的spawn还没结束
}