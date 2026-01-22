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
    "12forktest\0",
    "14forktest2\0",
    "filetest_simple\0",
    "ch6_file0\0",
    "ch6_file1\0",
    "ch6_file2\0",
    "ch6_file3\0",
];

/// 辅助测例，运行所有其他测例。

#[no_mangle]
extern "C" fn main() -> i32 {
    for test in TESTS {
        println!("Usertests: Running {}", test);
        let pid = spawn(*test);
        let mut xstate: i32 = Default::default();
        let wait_pid = waitpid(pid, &mut xstate);
        assert_eq!(pid, wait_pid);
        println!(
            "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
            test, pid, xstate
        );
    }
    println!("ch6 Usertests passed!");
    0
}
