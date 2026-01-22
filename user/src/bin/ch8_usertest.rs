#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{fork, exec, waitpid};

const TESTS: &[&str] = &[
    "00hello_world\0",
    "08power_3\0",
    "09power_5\0",
    "10power_7\0",
    "12forktest\0",
    "14forktest2\0",
    "filetest_simple\0",
    "threads\0",
    "threads_arg\0",
    "mpsc_sem\0",
    "sync_sem\0",
    "race_adder_mutex_blocking\0",
    "test_condvar\0",
    "ch8_deadlock_mutex1\0",
    "ch8_deadlock_sem1\0",
    "ch8_deadlock_sem2\0",
];

const TEST_NUM: usize = TESTS.len();

#[no_mangle]
extern "C" fn main() -> i32 {
    let mut pids = [0isize; TEST_NUM];
    for (i, &test) in TESTS.iter().enumerate() {
        println!("Usertests: Running {}", test);
        let pid = fork();
        if pid == 0 {
            exec(test);
            panic!("unreachable!");
        } else {
            pids[i] = pid;
        }
    }
    let mut xstate: i32 = Default::default();
    for (i, &test) in TESTS.iter().enumerate() {
        let wait_pid = waitpid(pids[i], &mut xstate);
        assert_eq!(pids[i], wait_pid);
        println!(
            "\x1b[32mUsertests: Test {} in Process {} exited with code {}\x1b[0m",
            test, pids[i], xstate
        );
    }
    println!("ch8 Usertests passed!");
    0
}
