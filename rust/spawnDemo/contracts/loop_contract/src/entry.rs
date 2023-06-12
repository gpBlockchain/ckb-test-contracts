// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::{vec, vec::Vec};

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    debug,
    high_level::{load_script, load_cell_data, load_tx_hash, QueryIter},
    ckb_types::{bytes::Bytes, prelude::*},
};
use ckb_std::ckb_constants::Source;

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    // remove below examples and write your code here

    let script = load_script()?;
    let args: Bytes = script.args().unpack();
    debug!("script args is {:?}", args);

    // return an error if args is invalid
    if args.is_empty() {
        return Err(Error::MyError);
    }
    let loopCount = collect_outputs_amount()?;
    // check cpu or mem
    if let Some(&first_byte) = args.first() {
        if first_byte == 1 {
            // mem
            if loopCount < 15000 {
                let a = vec![0; loopCount as usize];
                return Ok(());
            }
            let mut vec_arr = vec![];
            for i in (0..loopCount).step_by(100) {
                let a = vec![0; 100 as usize];
                vec_arr.push(a);
            }
            return Ok(());
        }
    }
    let ret = sum(loopCount);
    debug!("cpu:{:?}",ret);
    return Ok(());
}

fn sum(n: u128) -> u128 {
    let mut sum = 0;
    for i in 0..n {
        sum = sum + i;
    }
    return sum;
}

const UDT_LEN: usize = 16;

fn collect_outputs_amount() -> Result<u128, Error> {
    // With the sum of all input UDT tokens gathered, let's now iterate through
    // output cells to grab the sum of all output UDT tokens.
    let mut buf = [0u8; UDT_LEN];

    debug!("QueryIter:{:?}",QueryIter::new(load_cell_data, Source::GroupOutput).count());
    let udt_list = QueryIter::new(load_cell_data, Source::GroupOutput)
        .map(|data| {
            if data.len() == UDT_LEN {
                buf.copy_from_slice(&data);
                // u128 is 16 bytes
                Ok(u128::from_le_bytes(buf))
            } else {
                Err(Error::Encoding)
            }
        }).collect::<Result<Vec<_>, Error>>()?;
    Ok(udt_list.into_iter().sum::<u128>())
}

// Unit tests are supported.
#[test]
fn test_foo() {
    assert!(true);
}

