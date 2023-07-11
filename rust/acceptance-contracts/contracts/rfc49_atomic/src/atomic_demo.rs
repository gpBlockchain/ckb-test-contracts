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

use ckb_std::{debug};
use ckb_std::env::argv;
use ckb_std::syscalls::{set_content};
use core::sync::atomic::{AtomicBool, AtomicI32, Ordering};

///
/// use core/sync/atomic
///
pub fn program_entry() -> i8 {
    let value = AtomicBool::new(false);

    // 获取当前值
    let current_value = value.load(Ordering::SeqCst);
    debug!("Initial value: {}", current_value);

    // 如果当前值为false，则将其修改为true
    let new_value = true;
    let swapped = value.compare_and_swap(current_value, new_value, Ordering::SeqCst);

    // 检查交换是否成功
    if swapped == current_value {
        debug!("Value swapped successfully");
    } else {
        debug!("Value not swapped");
    }

    let counter = AtomicI32::new(0);

    // 原子地将计数器加1，并获取增加后的值
    let new_value = counter.fetch_add(1, Ordering::SeqCst) + 1;
    debug!("New value: {}", new_value);


    let counter = AtomicI32::new(10);

    // 原子地将计数器减1，并获取减少后的值
    let new_value = counter.fetch_sub(1, Ordering::SeqCst) - 1;
    debug!("New value: {}", new_value);

    return 0;
}
