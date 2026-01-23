#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::{exec, fork, wait};

#[no_mangle]
extern "C" fn main() -> i32 {
    if fork() == 0 {
        let mut target = match option_env!("CHAPTER").unwrap_or("0") {
            "5" => "ch5_usertest",
            "6" => "ch6_usertest",
            "8" => "ch8_usertest",
            _ => "",
        };
        if target.is_empty() || exec(target) == -1 {
            for candidate in ["ch5_usertest", "ch6_usertest", "ch8_usertest"] {
                if exec(candidate) != -1 {
                    target = candidate;
                    break;
                }
            }
            if target.is_empty() {
                exec("user_shell");
            }
        }
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                break;
            }
        }
    }
    0
}
