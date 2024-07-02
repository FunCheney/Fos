//! os/src/task/switch.rs
use super::TaskContext;
use core::arch::global_asm;

global_asm!(include_str!("switch.S"));
// 对于一般函数而言，Rust 编译器会在函数的起始位置自动生成代码来保存 s0 - s11 这些被调用着寄存器
// _switch 是一个用汇编语言写的特殊函数，它不会被 Rust 编译器所处理，所以需要在 _switch 中手动编写
// 保存 s0 - s11 的代码。
// 调用该函数来完成切换功能而不是直接跳转到符号 _switch 的地址。
// Rust 编译器在调用前后会自动插入保存/恢复 调用者寄存器的代码。
extern "C" {
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}
