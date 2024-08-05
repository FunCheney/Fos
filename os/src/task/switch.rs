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
    /// current_task_cx_ptr: 当前任务的上下文地址
    /// next_task_cx_ptr: 下一个任务的上下文地址
    /// 任务切换分为如下四个步骤
    /// 1. Trap 控制流 A 调用 __switch 之前，A 的内核上栈上只有 Trap 上下文，和 Trap 处理函数的调用
    ///    栈信息，而 B 是被之前切换出去的。
    /// 2. A 在A 的上下文空间里面保存 CPU 当前寄存器快照
    /// 3. 读取 next_task_cx_ptr 指向的 B 任务上下文，根据 B 任务上下文报错的内容来恢复 ra 寄存器，
    ///    s0 - s11 寄存器以及 sp 寄存器。只有做完这一步 __switch 才能做到一个函数夸两条控制流执行，
    ///    即 通过换栈也就实现了控制流的切换
    /// 4. 上一步寄存器恢复完成之后，通过恢复 sp 寄存器换到了任务 B 的内核栈上，进而实现了控制流的
    ///    切换。
    pub fn __switch(current_task_cx_ptr: *mut TaskContext, next_task_cx_ptr: *const TaskContext);
}
