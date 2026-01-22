#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{spawn, waitpid};

static TESTS: &[&str] = &[
    "00hello_world\0",
    "08power_3\0",
    "09power_5\0",
    "10power_7\0",
    "ch3_sleep\0",
    "ch3_sleep1\0",
    "ch4_mmap\0",
    "ch4_mmap1\0",
    "ch4_mmap2\0",
    "ch4_mmap3\0",
    "ch4_unmap\0",
    "ch4_unmap2\0",
    "ch5_spawn0\0",
    "ch5_spawn1\0",
    "ch5_setprio\0",
];

static STEST: &str = "ch5_stride\0";

/// 辅助测例，运行所有其他测例。

#[no_mangle]
extern "C" fn main() -> i32 {
    let mut pid = [0isize; 20];
    for (i, &test) in TESTS.iter().enumerate() {
        println!("Usertests: Running {}", test);
        pid[i] = spawn(test);
    }
    let mut xstate: i32 = Default::default();
    for (i, &test) in TESTS.iter().enumerate() {
        let wait_pid = waitpid(pid[i], &mut xstate);
        println!(
            "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
            test, pid[i], xstate
        );
        assert_eq!(pid[i], wait_pid);
    }
    println!("Usertests: Running {}", STEST);
    let spid = spawn(STEST);
    xstate = Default::default();
    let wait_pid = waitpid(spid, &mut xstate);
    assert_eq!(spid, wait_pid);
    println!(
        "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
        STEST, spid, xstate
    );
    println!("ch5 Usertests passed!");
    0
}
