# 第一章

第一章旨在展示一个尽量简单的**特权态裸机应用程序**：

- 核心逻辑在 [main.rs](src/main.rs)；
- 链接脚本在 [build.rs](build.rs)，以免增加依赖；
- 为了支持 nobios 模式和 RV32 架构，还包含：
  - `sbi.rs` - SBI 服务调用实现
  - `msbi.rs` - M-Mode SBI 处理器（nobios 模式）
  - `m_entry_rv64.asm` / `m_entry_rv32.asm` - M-Mode 入口汇编代码（nobios 模式）
- 支持两种启动模式：
  - **SBI 模式**（默认）：程序被 SEE 引导，工作在 S 态；
  - **nobios 模式**：内核直接启动，包含 M-Mode 入口代码，自行处理 M-Mode 到 S-Mode 的切换；
- 支持两种架构：
  - **RV64**（默认）：64 位 RISC-V 架构（riscv64gc）
  - **RV32**：32 位 RISC-V 架构（riscv32imac），仅支持 nobios 模式
- 这个程序不需要环境：
  - 从汇编进入并为 Rust 准备栈；
  - 依赖 SBI 提供的 `console_putchar` 打印 `Hello, world!`；
  - 依赖 SBI 提供的 `shutdown` 调用关机；

它不配被称作一个操作系统，因为它没有操作（硬件），也不构造（执行用户程序的）系统；

## 运行方式

### SBI 模式（默认）
```bash
cargo qemu --ch 1
```

### nobios 模式
```bash
cargo qemu --ch 1 --nobios
```

### RV32 nobios 模式
```bash
cargo qemu --ch 1 --arch riscv32 --nobios
```

## SBI 实现

本章使用自定义的 SBI 实现（`sbi.rs`）。

- **SBI 模式**：通过 `ecall` 调用 RustSBI 提供的服务
- **nobios 模式**：通过 `ecall` 触发 M-Mode 陷阱，由 `msbi.rs` 中的 `m_trap_handler` 处理 SBI 调用

在 nobios 模式下，`msbi.rs` 实现了最小化的 M-Mode SBI，提供：
- `console_putchar`：通过 UART MMIO 输出字符
- `shutdown`：通过 QEMU 测试设备关机

## 定制链接脚本

build.rs 的用法见[文档](https://doc.rust-lang.org/cargo/reference/build-scripts.html)。根据是否启用 `nobios` feature，会生成不同的链接脚本：

### SBI 模式链接脚本

```ld
OUTPUT_ARCH(riscv)
SECTIONS {
    .text 0x80200000 : {
        *(.text.entry)
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    .bss : {
        *(.bss.uninit)
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
}
```

### nobios 模式链接脚本

nobios 模式在 `0x80000000` 处开始，包含 M-Mode 入口代码、陷阱处理、栈和数据段，然后跳转到 `0x80200000` 处开始执行 S-Mode 内核代码。

1. 为了被引导，它的 `.text` 在最前面。一般是 `.rodata` 在最前面。`.text` 的最前面是 `.text.entry`，有且只有一个汇编入口放在这个节，实现引导；
2. 正常情况下，裸机应用程序需要清除自己的 `.bss` 节，所以需要定义全局符号以便动态定位 `.bss`。但这一章的程序并不依赖 清空的 `.bss`，所以没有导出符号。`.bss` 本身仍然需要，因为栈会放在里面。

## 工作流程解读

### SBI 模式

1. SBI 初始化完成后，将固定跳转到 0x8020_0000 地址；
2. 根据链接脚本，汇编入口函数被放置在这个地址。它叫做 `_start`，这个名字是特殊的！GNU LD 及兼容其脚本的链接器会将这个名字认为是默认的入口，否则需要指定。这个函数是一个 rust 裸函数（[`#[naked]`](https://github.com/rust-lang/rust/issues/90957)），编译器不会为它添加任何序言和尾声，因此可以在没有栈的情况下执行。它将栈指针指向预留的栈空间，然后跳转到 `rust_main` 函数；
3. `rust_main` 函数在一个最简单的循环打印调用 sbi 打印 `Hello, world!` 字符串，然后关机。

### nobios 模式

1. QEMU 使用 `-bios none` 参数，直接从 `0x80000000` 加载内核；
2. M-Mode 入口代码（`_m_start`，在 `m_entry_rv64.asm` 或 `m_entry_rv32.asm` 中）在 `0x80000000` 执行；
3. M-Mode 设置 PMP（物理内存保护）、委托陷阱、配置 CSR（控制状态寄存器），然后通过 `mret` 跳转到 S-Mode；
4. S-Mode 内核在 `0x80200000` 开始执行，入口函数为 `_start`；
5. 当 S-Mode 调用 SBI 服务时，通过 `ecall` 触发 M-Mode 陷阱；
6. M-Mode 陷阱处理器（`m_trap_handler`，在 `msbi.rs` 中）处理 SBI 调用并返回结果。
