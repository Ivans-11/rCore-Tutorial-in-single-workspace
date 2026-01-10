//! M-Mode SBI 实现
//!
//! 在 nobios 模式下，提供一个最小的 SBI 实现，处理 S-Mode 的 ecall。

/// QEMU virt UART 基地址
const UART_BASE: usize = 0x1000_0000;

/// UART 操作 (16550 兼容)
mod uart {
    use super::UART_BASE;

    const THR: usize = UART_BASE;     // Transmit Holding Register
    const LSR: usize = UART_BASE + 5; // Line Status Register

    /// 检查 UART 是否准备好发送
    #[inline]
    fn is_tx_ready() -> bool {
        unsafe {
            let lsr = (LSR as *const u8).read_volatile();
            (lsr & 0x20) != 0 // THRE bit
        }
    }

    /// 写入一个字节到 UART
    pub fn putchar(c: u8) {
        while !is_tx_ready() {}
        unsafe {
            (THR as *mut u8).write_volatile(c);
        }
    }
}

/// SBI Extension IDs
mod eid {
    pub const LEGACY_CONSOLE_PUTCHAR: usize = 0x01;
    pub const LEGACY_SHUTDOWN: usize = 0x08;
    pub const BASE: usize = 0x10;
    pub const SRST: usize = 0x53525354;
}

/// SBI 错误码
mod error {
    pub const SUCCESS: isize = 0;
    pub const ERR_NOT_SUPPORTED: isize = -2;
}

/// SBI 返回值
#[repr(C)]
pub struct SbiRet {
    pub error: isize,
    pub value: usize,
}

impl SbiRet {
    fn success(value: usize) -> Self {
        SbiRet { error: error::SUCCESS, value }
    }

    fn not_supported() -> Self {
        SbiRet { error: error::ERR_NOT_SUPPORTED, value: 0 }
    }
}

/// 处理 legacy console putchar (EID 0x01)
fn handle_console_putchar(c: usize) -> SbiRet {
    uart::putchar(c as u8);
    SbiRet::success(0)
}

/// 处理系统复位
fn handle_system_reset(reset_reason: usize) -> SbiRet {
    const VIRT_TEST: usize = 0x10_0000;
    const FINISHER_PASS: u32 = 0x5555;
    const FINISHER_FAIL: u32 = 0x3333;

    let code = if reset_reason == 0 { FINISHER_PASS } else { FINISHER_FAIL };
    unsafe {
        (VIRT_TEST as *mut u32).write_volatile(code);
    }
    loop {}
}

/// 处理 legacy shutdown (EID 0x08)
fn handle_legacy_shutdown() -> SbiRet {
    handle_system_reset(0)
}

/// 处理 SBI base 扩展 (EID 0x10)
fn handle_base(fid: usize) -> SbiRet {
    match fid {
        0 => SbiRet::success(2), // spec_version: SBI 0.2
        1 => SbiRet::success(0), // impl_id
        2 => SbiRet::success(1), // impl_version
        3 => SbiRet::success(1), // probe_extension
        4 => SbiRet::success(0), // mvendorid
        5 => SbiRet::success(0), // marchid
        6 => SbiRet::success(0), // mimpid
        _ => SbiRet::not_supported(),
    }
}

/// M-Mode trap handler，由汇编调用
///
/// 参数通过寄存器传递：
/// - a0-a5: SBI 调用参数
/// - a6: FID (function ID)
/// - a7: EID (extension ID)
#[unsafe(no_mangle)]
pub extern "C" fn m_trap_handler(
    a0: usize,
    a1: usize,
    _a2: usize,
    _a3: usize,
    _a4: usize,
    _a5: usize,
    fid: usize,
    eid: usize,
) -> SbiRet {
    // 检查 mcause，只处理来自 S-Mode 的 ecall (cause = 9)
    let mcause: usize;
    unsafe {
        core::arch::asm!("csrr {}, mcause", out(reg) mcause);
    }

    if mcause != 9 {
        return SbiRet::not_supported();
    }

    // 根据 EID 处理 SBI 调用
    match eid {
        eid::LEGACY_CONSOLE_PUTCHAR => handle_console_putchar(a0),
        eid::LEGACY_SHUTDOWN => handle_legacy_shutdown(),
        eid::BASE => handle_base(fid),
        eid::SRST => {
            if fid == 0 {
                handle_system_reset(a1)
            } else {
                SbiRet::not_supported()
            }
        }
        _ => SbiRet::not_supported(),
    }
}
