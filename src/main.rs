#![feature(wasi_ext)]
use ::wasi::wasi_unstable as wasi;
use std::os::wasi::io::AsRawFd;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let options = ["thread_sleep", "poll_clock", "poll_stdin_read", "poll_fd_read"];

    match args.get(1).map(|s| s.as_str()) {
        Some("thread_sleep") => std::thread::sleep(std::time::Duration::new(0,5)),
        Some("poll_clock") => debug_std_thread_sleep(),
        Some("poll_stdin_read") => try_stdin_read_poll_oneoff(),
        Some("poll_fd_read") => try_fd_read_poll_oneoff(),
        _ => {
            println!("WASI test for poll_oneoff function\n");
            println!("Usage: poll_oneoff_tests.wasm <test>; where test is one of: {:?}", options)
        }
    }
}

fn debug_std_thread_sleep() {
    // Copied from thread::sleep
    const CLOCK_ID: wasi::Userdata = 0x0123_45678;

    let clock = wasi::raw::__wasi_subscription_u_clock_t {
        identifier: CLOCK_ID,
        clock_id: wasi::CLOCK_MONOTONIC,
        timeout: 5u64,
        precision: 0,
        flags: 0,
    };
    let in_ = [wasi::Subscription {
        userdata: 0, //Should be CLOCK_ID
        type_: wasi::EVENTTYPE_CLOCK,
        u: wasi::raw::__wasi_subscription_u { clock: clock },
    }];
    let (res, event) = unsafe {
        let mut out: [wasi::Event; 1] = std::mem::zeroed();
        let res = wasi::poll_oneoff(&in_, &mut out);
        (res, out[0])
    };

    println!("Res = {:?}", res);
    assert_eq!(event.userdata, CLOCK_ID, "event.userdata");
    assert_eq!(event.error, 0, "event.error");
    assert_eq!(event.type_, wasi::EVENTTYPE_CLOCK, "event.type_");
}


fn try_stdin_read_poll_oneoff() {
    // Copied from thread::sleep
    let fd_readwrite = wasi::raw::__wasi_subscription_u_fd_readwrite_t {
        fd: 1
    };
    let in_ = [wasi::Subscription {
        userdata: 1,
        type_: wasi::EVENTTYPE_FD_READ,
        u: wasi::raw::__wasi_subscription_u { fd_readwrite },
    }];
    let (res, event) = unsafe {
        let mut out: [wasi::Event; 1] = std::mem::zeroed();
        let res = wasi::poll_oneoff(&in_, &mut out);
        (res, out[0])
    };

    println!("Res = {:?}", res);
    assert_eq!(event.userdata, 1, "event.userdata");
    assert_eq!(event.error, 0, "event.error");
    assert_eq!(event.type_, wasi::EVENTTYPE_FD_READ, "event.type_");
}

fn try_fd_read_poll_oneoff() {
    let mut input = std::fs::File::open("input.txt").expect("file input.txt");
    let mut buf: String = String::new();
    println!("{:?}", input.read_to_string(&mut buf));

    println!("File descriptor: {}", input.as_raw_fd());

    let fd_readwrite = wasi::raw::__wasi_subscription_u_fd_readwrite_t {
        fd: input.as_raw_fd()
    };
    let in_ = [wasi::Subscription {
        userdata: 1,
        type_: wasi::EVENTTYPE_FD_READ,
        u: wasi::raw::__wasi_subscription_u { fd_readwrite },
    }];
    let (res, event) = unsafe {
        let mut out: [wasi::Event; 1] = std::mem::zeroed();
        let res = wasi::poll_oneoff(&in_, &mut out);
        (res, out[0])
    };

    println!("Res = {:?}", res);
    assert_eq!(event.userdata, 1, "event.userdata");
    assert_eq!(event.error, 0, "event.error");
    assert_eq!(event.type_, wasi::EVENTTYPE_FD_READ, "event.type_");
}