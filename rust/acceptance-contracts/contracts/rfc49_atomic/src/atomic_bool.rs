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

use core::sync::atomic::{AtomicBool, Ordering};

///
/// AtomicBool example
///
///
pub fn program_entry() -> i8 {

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicBool.html#method.new
    debug!("new");
    let atomic_true = AtomicBool::new(true);
    let atomic_false = AtomicBool::new(false);

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicBool.html#method.get_mut
    debug!("get_mut");
    let mut some_bool = AtomicBool::new(true);
    assert_eq!(*some_bool.get_mut(), true);
    *some_bool.get_mut() = false;
    assert_eq!(some_bool.load(Ordering::SeqCst), false);

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicBool.html#method.into_inner

    let some_bool = AtomicBool::new(true);
    assert_eq!(some_bool.into_inner(), true);

    // https://doc.rust-lang.org/core/sync/atomic/struct.AtomicBool.html#method.load
    debug!("load");
    let some_bool = AtomicBool::new(true);
    assert_eq!(some_bool.load(Ordering::Relaxed), true);

    // https://doc.rust-lang.org/core/sync/atomic/struct.AtomicBool.html#method.store
    debug!("store");
    let some_bool = AtomicBool::new(true);

    some_bool.store(false, Ordering::Relaxed);
    assert_eq!(some_bool.load(Ordering::Relaxed), false);

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicBool.html#method.swap
    debug!("swap");

    let some_bool = AtomicBool::new(true);

    assert_eq!(some_bool.swap(false, Ordering::Relaxed), true);
    assert_eq!(some_bool.load(Ordering::Relaxed), false);



    return 0;
}
