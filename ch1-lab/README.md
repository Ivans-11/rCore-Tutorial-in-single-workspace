# 第一章实验

第一章实验的示例，展示如何依赖 `rcore_console` crate。

在 [Cargo.toml](Cargo.toml#L12) 里添加：

```toml
rcore_console = { path = "../rcore_console"}
```

在 [main.rs](src/main.rs#L47) 里初始化：

```rust
rcore_console::init_console(&Console);
rcore_console::set_timestamp(timer::get_time_ms);
```

## 时间戳功能

本章实现了 `timer.rs` 模块，提供 `get_time_ms()` 函数用于获取当前时间（毫秒）。通过 `rcore_console::set_timestamp()` 注册后，所有 `println!` 输出都会自动在开头显示时间戳，格式为 `[    X ms]`。

## 运行方式

### SBI 模式（默认）
```bash
cargo qemu --ch 1 --lab
```

### nobios 模式
```bash
cargo qemu --ch 1 --lab --nobios
```

### RV32 nobios 模式
```bash
cargo qemu --ch 1 --lab --arch riscv32 --nobios
```

后续的章节都可以这样依赖 `rcore_console`。
