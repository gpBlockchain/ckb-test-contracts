// Import from `core` instead of from `std` since we are in no-std mode
use crate::error::Error;
use alloc::vec;
use ckb_std::syscalls;
use core::result::Result;

pub fn main() -> Result<(), Error> {
    let argv = ckb_std::env::argv();
    let mut std_fds: [u64; 2] = [0; 2];
    syscalls::inherited_file_descriptors(&mut std_fds);

    // Create a very large buffer
    //https://github.com/nervosnetwork/ckb-std/blob/ac1eea0203b8fe37b6b98efb0ea3618d436d7ebe/src/global_alloc_macro/default_alloc.rs#L23
    let mut out = vec![0u8; 512 * 1024]; // 512 KB buffer

    // Fill the buffer with data from argv
    let mut pos = 0;
    for arg in argv {
        let arg_bytes = arg.to_bytes();
        if pos + arg_bytes.len() > out.len() {
            // If buffer overflow is about to happen, stop filling
            break;
        }
        out[pos..pos + arg_bytes.len()].copy_from_slice(arg_bytes);
        pos += arg_bytes.len();
    }

    // Try writing the buffer
    let len = syscalls::write(std_fds[1], &out)?;

    // Just for example, we won't assert the length here because we are writing a huge buffer
    // assert_eq!(len, 10);

    Ok(())
}
