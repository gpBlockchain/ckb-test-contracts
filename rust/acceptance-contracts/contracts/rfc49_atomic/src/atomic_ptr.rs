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

use core::sync::atomic::{AtomicPtr, Ordering};


pub fn program_entry() -> i8 {
    let ptr = &mut 5;
    let atomic_ptr = AtomicPtr::new(ptr);


    let mut data = 10;
    let mut atomic_ptr = AtomicPtr::new(&mut data);
    let mut other_data = 5;
    *atomic_ptr.get_mut() = &mut other_data;
    assert_eq!(unsafe { *atomic_ptr.load(Ordering::SeqCst) }, 5);


    let mut data = 5;
    let atomic_ptr = AtomicPtr::new(&mut data);
    assert_eq!(unsafe { *atomic_ptr.into_inner() }, 5);


    let ptr = &mut 5;
    let some_ptr = AtomicPtr::new(ptr);

    let value = some_ptr.load(Ordering::Relaxed);


    let ptr = &mut 5;
    let some_ptr = AtomicPtr::new(ptr);

    let other_ptr = &mut 10;

    some_ptr.store(other_ptr, Ordering::Relaxed);


    let ptr = &mut 5;
    let some_ptr = AtomicPtr::new(ptr);

    let other_ptr = &mut 10;

    let value = some_ptr.swap(other_ptr, Ordering::Relaxed);


    let ptr = &mut 5;
    let some_ptr = AtomicPtr::new(ptr);

    let other_ptr = &mut 10;

    let value = some_ptr.compare_and_swap(ptr, other_ptr, Ordering::Relaxed);


    let ptr = &mut 5;
    let some_ptr = AtomicPtr::new(ptr);

    let other_ptr = &mut 10;

    let value = some_ptr.compare_exchange(ptr, other_ptr,
                                          Ordering::SeqCst, Ordering::Relaxed);


    let some_ptr = AtomicPtr::new(&mut 5);

    let new = &mut 10;
    let mut old = some_ptr.load(Ordering::Relaxed);
    loop {
        match some_ptr.compare_exchange_weak(old, new, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(_) => break,
            Err(x) => old = x,
        }
    }

    let ptr: *mut _ = &mut 5;
    let some_ptr = AtomicPtr::new(ptr);

    let new: *mut _ = &mut 10;
    assert_eq!(some_ptr.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |_| None), Err(ptr));
    let result = some_ptr.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |x| {
        if x == ptr {
            Some(new)
        } else {
            None
        }
    });
    assert_eq!(result, Ok(ptr));
    assert_eq!(some_ptr.load(Ordering::SeqCst), new);

    return 0;
}