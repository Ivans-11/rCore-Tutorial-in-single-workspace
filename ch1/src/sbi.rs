//! SBI 服务调用封装
//!
//! 不依赖 sbi_rt，直接通过 ecall 调用 SBI 服务。

use core::arch::asm;

/// SBI 调用封装
#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> (isize, usize) {
    let error: isize;
    let value: usize;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a2") arg2,
            in("a6") fid,
            in("a7") eid,
        );
    }
    (error, value)
}

/// SBI Extension IDs
const SBI_CONSOLE_PUTCHAR: usize = 0x01;
const SBI_SHUTDOWN: usize = 0x08;
const SBI_SRST: usize = 0x53525354;

/// 输出一个字符到控制台
pub fn console_putchar(c: u8) {
    sbi_call(SBI_CONSOLE_PUTCHAR, 0, c as usize, 0, 0);
}

/// 关机
pub fn shutdown(failure: bool) -> ! {
    // 尝试 SRST 扩展 (SBI v0.3+)
    let reason = if failure { 1usize } else { 0usize };
    sbi_call(SBI_SRST, 0, 0, reason, 0);

    // 回退到 legacy shutdown
    sbi_call(SBI_SHUTDOWN, 0, 0, 0, 0);

    loop {}
}
