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

use core::sync::atomic::{AtomicUsize, Ordering};

///
/// AtomicUsize example
///     https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.get_mut
///     https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.into_inner
///     https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.store
///     https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.swap
///     https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.compare_and_swap
///
///
///
pub fn program_entry() -> i8 {
    let atomic_forty_two = AtomicUsize::new(42);

    // https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.get_mut
    debug!("get_mut");
    let mut some_var = AtomicUsize::new(10);
    assert_eq!(*some_var.get_mut(), 10);
    *some_var.get_mut() = 5;
    assert_eq!(some_var.load(Ordering::SeqCst), 5);


    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.into_inner
    debug!("into_inner");
    let some_var = AtomicUsize::new(5);
    assert_eq!(some_var.into_inner(), 5);


    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.store
    debug!("store");
    let some_var = AtomicUsize::new(5);
    some_var.store(10, Ordering::Relaxed);
    assert_eq!(some_var.load(Ordering::Relaxed), 10);

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.swap
    debug!("swap");
    let some_var = AtomicUsize::new(5);
    assert_eq!(some_var.swap(10, Ordering::Relaxed), 5);


    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.compare_and_swap
    debug!("compare_and_swap");
    let some_var = AtomicUsize::new(5);

    assert_eq!(some_var.compare_and_swap(5, 10, Ordering::Relaxed), 5);
    assert_eq!(some_var.load(Ordering::Relaxed), 10);

    assert_eq!(some_var.compare_and_swap(6, 12, Ordering::Relaxed), 10);
    assert_eq!(some_var.load(Ordering::Relaxed), 10);

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.compare_exchange
    debug!("compare_exchange");
    let some_var = AtomicUsize::new(5);

    assert_eq!(some_var.compare_exchange(5, 10,
                                         Ordering::Acquire,
                                         Ordering::Relaxed),
               Ok(5));
    assert_eq!(some_var.load(Ordering::Relaxed), 10);

    assert_eq!(some_var.compare_exchange(6, 12,
                                         Ordering::SeqCst,
                                         Ordering::Acquire),
               Err(10));
    assert_eq!(some_var.load(Ordering::Relaxed), 10);

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.compare_exchange_weak

    // let mut old = val.load(Ordering::Relaxed);
    // loop {
    //     let new = old * 2;
    //     match val.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
    //         Ok(_) => break,
    //         Err(x) => old = x,
    //     }
    // }

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.fetch_add
    debug!("fetch_add");
    let foo = AtomicUsize::new(0);
    assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);
    assert_eq!(foo.load(Ordering::SeqCst), 10);

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.fetch_sub
    debug!("fetch_sub");
    let foo = AtomicUsize::new(20);
    assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);
    assert_eq!(foo.load(Ordering::SeqCst), 10);

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.fetch_and
    debug!("fetch_and");
    let foo = AtomicUsize::new(0b101101);
    assert_eq!(foo.fetch_and(0b110011, Ordering::SeqCst), 0b101101);
    assert_eq!(foo.load(Ordering::SeqCst), 0b100001);

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.fetch_nand
    debug!("fetch_nand");
    let foo = AtomicUsize::new(0x13);
    assert_eq!(foo.fetch_nand(0x31, Ordering::SeqCst), 0x13);
    assert_eq!(foo.load(Ordering::SeqCst), !(0x13 & 0x31));

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.fetch_or
    debug!("fetch_or");
    let foo = AtomicUsize::new(0b101101);
    assert_eq!(foo.fetch_or(0b110011, Ordering::SeqCst), 0b101101);
    assert_eq!(foo.load(Ordering::SeqCst), 0b111111);

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.fetch_xor
    debug!("fetch_xor");
    let foo = AtomicUsize::new(0b101101);
    assert_eq!(foo.fetch_xor(0b110011, Ordering::SeqCst), 0b101101);
    assert_eq!(foo.load(Ordering::SeqCst), 0b011110);

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.fetch_update
    debug!("fetch_update");
    let x = AtomicUsize::new(7);
    assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(7));
    assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(7));
    assert_eq!(x.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| Some(x + 1)), Ok(8));
    assert_eq!(x.load(Ordering::SeqCst), 9);

    // https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.fetch_max
    debug!("fetch_max");
    let foo = AtomicUsize::new(23);
    assert_eq!(foo.fetch_max(42, Ordering::SeqCst), 23);
    assert_eq!(foo.load(Ordering::SeqCst), 42);

    let foo = AtomicUsize::new(23);
    let bar = 42;
    let max_foo = foo.fetch_max(bar, Ordering::SeqCst).max(bar);
    assert!(max_foo == 42);

    //https://doc.rust-lang.org/core/sync/atomic/struct.AtomicUsize.html#method.fetch_min
    debug!("fetch_min");
    let foo = AtomicUsize::new(23);
    assert_eq!(foo.fetch_min(42, Ordering::Relaxed), 23);
    assert_eq!(foo.load(Ordering::Relaxed), 23);
    assert_eq!(foo.fetch_min(22, Ordering::Relaxed), 23);
    assert_eq!(foo.load(Ordering::Relaxed), 22);


    let foo = AtomicUsize::new(23);
    let bar = 12;
    let min_foo = foo.fetch_min(bar, Ordering::SeqCst).min(bar);
    assert_eq!(min_foo, 12);

    return 0;
}
