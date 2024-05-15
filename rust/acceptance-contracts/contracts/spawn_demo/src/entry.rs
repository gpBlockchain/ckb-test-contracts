// Import from `core` instead of from `std` since we are in no-std mode
use crate::error::Error;
use alloc::{format};
use alloc::string::ToString;
use ckb_std::syscalls;
use core::result::Result;
use ckb_std::error::SysError;
use core::ffi::CStr;
use ckb_std::ckb_constants::Source;
use ckb_std::env::argv;
use ckb_std::high_level::spawn_cell;


pub fn main() -> Result<(), Error> {
    let argvs = argv();
    // debug!("argvs length:{:?}:{:?}",argvs.len(),argvs);

    if argvs.len() != 0 {
        let process_id = syscalls::process_id();
        let mut std_fds: [u64; 2] = [0, 0];
        syscalls::inherited_file_descriptors(&mut std_fds);

        syscalls::write(std_fds[1], &[1u8, 1u8, 1u8, 1u8])?;
        syscalls::write(std_fds[1], &[2u8, 2u8])?;

        // syscalls::close(std_fds[1])?;

        let mut read: [u8; 4] = [0, 0, 0, 0];
        // syscalls::read(std_fds[0], &mut read)?;
        // syscalls::debug(format!("spawn read:{:?}", read));
        syscalls::debug(format!("spawn process_id:{:?}", process_id));
        // match syscalls::wait(process_id) {
        //     Ok(ret) => {
        //         syscalls::debug(format!("spawn wait ret:{:?}", ret));
        //     }
        //     Err(err) => {
        //         syscalls::debug(format!("spawn wait err:{:?}", err));
        //     }
        // };
        syscalls::debug(format!("spawn std_fds result:{:?}", std_fds));
        for i in 0..2 {
            syscalls::debug(format!("spawn pipe:{:?}", i));

            match syscalls::pipe() {
                Ok((r0, w0)) => {
                    syscalls::debug(format!("spawn pipe r0:{:?},w0:{:?}", r0, w0));
                    syscalls::close(r0)?;
                }
                Err(err) => {
                    syscalls::debug(format!("spawn err:{:?}", err));
                }
            };
        }
        match syscalls::close(1) {
            Ok(_) => {
                syscalls::debug(format!("close result:ok"));
            }
            Err(failed) => {
                syscalls::debug(format!("close result error:{:?}", failed));
            }
        };
        syscalls::debug(format!("argvs.len() != 0"));
        for i in 0..5 {
            syscalls::debug(format!("for :{:?}", i));
        }
        return Ok(());
    }

    if argvs.len() == 0 {
        syscalls::debug(format!("argvs.len() == 0"));
    }

    let argc: u64 = 2;
    let argv = {
        let mut argv = alloc::vec![core::ptr::null(); argc as usize + 1];
        argv[0] = CStr::from_bytes_with_nul(b"hello\0").unwrap().as_ptr();
        argv[1] = CStr::from_bytes_with_nul(b"world\0").unwrap().as_ptr();
        argv
    };

    let mut std_fds: [u64; 2] = [0, 0];
    let mut son_fds: [u64; 3] = [0, 0, 0];


    let (r0, w0) = syscalls::pipe()?;
    std_fds[0] = r0;
    son_fds[1] = w0;
    // match syscalls::write(w0, &[1u8, 2u8, 3u8, 4u8]) {
    //     Ok(ok) => {
    //         syscalls::debug(format!("write:{:?}",ok));
    //     }
    //     Err(err) => {
    //         syscalls::debug(format!("write err:{:?}",err));
    //     }
    // };
    //
    // let mut read: [u8; 4] = [0, 0, 0, 0];
    // syscalls::read(r0, &mut read)?;
    // syscalls::debug(format!("first std_fds[0] read:{:?}", read));

    let (r1, w1) = syscalls::pipe()?;
    std_fds[1] = w1;
    son_fds[0] = r1;
    let mut pid: u64 = 0;
    let mut spgs = syscalls::SpawnArgs {
        argc: argc,
        argv: argv.as_ptr(),
        process_id: &mut pid as *mut u64,
        inherited_fds: son_fds.as_ptr(),
    };
    for i in 0..1 {
        syscalls::debug(format!("pipe:{:?}", i));
        match syscalls::pipe() {
            Ok((r0, w0)) => {
                syscalls::debug(format!("pipe r0:{:?},w0:{:?}", r0, w0));
            }
            Err(err) => {
                syscalls::debug(format!("err:{:?}", err));
            }
        };
    }
    let spawn_result1 = syscalls::spawn(0, Source::CellDep, 0, 0, &mut spgs)?;


    syscalls::debug("------- read -----------".to_string());


    let mut read: [u8; 4] = [0, 0, 0, 0];
    syscalls::read(std_fds[0], &mut read)?;
    syscalls::debug(format!("std_fds[0] read:{:?}", read));
    let mut read: [u8; 2] = [0, 0];
    match syscalls::read(std_fds[0], &mut read) {
        Ok(ret) => {
            syscalls::debug(format!("std_fds[0] read size :{:?}", ret));
        }
        Err(err) => {
            syscalls::debug(format!("std_fds[0] read err :{:?}", err));
        }
    };
    syscalls::debug(format!("std_fds[0] again read:{:?}", read));
    match syscalls::write(std_fds[1], &[3u8, 1u8, 1u8]) {
        Ok(ok) => {
            syscalls::debug(format!("sys calls::write:{:?}", ok));
        }
        Err(err) => {
            syscalls::debug(format!("sys calls::err:{:?}", err));
        }
    };

    match syscalls::read(std_fds[0], &mut read) {
        Ok(ret) => {
            syscalls::debug(format!("std_fds[0] read size :{:?}", ret));
        }
        Err(err) => {
            syscalls::debug(format!("std_fds[0] read err :{:?}", err));
        }
    };

    syscalls::debug(format!("spawn1  result:{:?}", spawn_result1));
    syscalls::debug(format!("spawn1  pid:{:?}", pid));
    let mut pid: u64 = 0;
    let mut std_fds: [u64; 2] = [0, 0];
    let mut son_fds: [u64; 3] = [0, 0, 0];
    let (r0, w0) = syscalls::pipe()?;
    syscalls::debug(format!("pipe r0:{:?},w0:{:?}", r0, w0));
    std_fds[0] = r0;
    son_fds[1] = w0;
    let (r1, w1) = syscalls::pipe()?;
    syscalls::debug(format!("pipe r0:{:?},w0:{:?}", r1, w1));
    std_fds[1] = w1;
    son_fds[0] = r1;
    let mut spgs = syscalls::SpawnArgs {
        argc: argc,
        argv: argv.as_ptr(),
        process_id: &mut pid as *mut u64,
        inherited_fds: son_fds.as_ptr(),
    };
    let spawn_result2 = syscalls::spawn(0, Source::CellDep, 0, 0, &mut spgs)?;
    syscalls::debug(format!("spawn2 result:{:?}", spawn_result2));

    let mut buf: [u8; 256] = [0; 256];
    // let len = syscalls::read(std_fds[0], &mut buf)?;
    // syscalls::debug(format!("read len:{:?}",len));
    // assert_eq!(len, 10);
    // buf[len] = 0;
    // assert_eq!(
    //     CStr::from_bytes_until_nul(&buf).unwrap().to_str().unwrap(),
    //     "helloworld"
    // );
    Ok(())
}