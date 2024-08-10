use core::arch::asm;

use super::task::TaskInfo;
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_YIELD: usize = 124;
pub const SYSCALL_GET_TIME: usize = 169; 
pub const SYSCALL_TASK_INFO: usize = 410;
/// 功能: 将系统调用封装成 syscall 函数
/// 参数: 'id' 系统调用id 
///       'args' 三个参数
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe{
        // 使用 asm 嵌入 ecall 指令来触发系统调用
        // asm 宏可以获取上下文中的参数信息，并允许嵌入的汇编代码对这些参数
        // 进行操作。由于编译器的能力不足以判断汇编代码的安全性，所以需要包裹在
        // unsafe 块中，我们自己对他负责。
        asm!(
            //汇编 代码 应用程序通过 ecall 调用操作系统提供的接口
            "ecall",
            // a0 比较特殊，同时用作输入/输出 
            inlateout("x10") args[0] => ret,
            // 将输入参数 args[1] 绑定到 ecall 的输入寄存器 x11 中，即 a1 寄存器中
            // 编译器自动插入相关指令，并保证在 ecall 执行前寄存器 a1 的值与 args[1] 相同
            in("x11") args[1],
            // 同理，将 args[2], id 分别绑定到 a2, a7 寄存器中
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

/// 功能： 将内存中缓冲区中的数据写入文件
/// 参数： 'fd' 表示要写入的文件描述符
///        'buf' 表示内存中缓冲区的起始地址 &[u8] 切片类型用来描述缓冲区，这是一个
///              胖指针，里面包含缓冲区的齐始地址，长度，通过 as_ptr, len 取出他们作为独立的系统参
///              数调用。
///        'len' 表示内存中缓冲区的长度
/// 返回值： 返回成功写入的长度
/// syscall ID: 64
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd,buffer.as_ptr() as usize, buffer.len()])
}


/// 功能: 退出应用程序并将返回值告知批处理系统
/// 参数: 'exit_code' 表示应用程序的返回值 
/// 返回值: 该系统调用不应该返回
/// syscall ID: 93
pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

/// 功能: 应用主动交出 CPU 所有权并切换到其他应用
/// 返回值: 总是返回 0
/// syscall ID: 124
pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0,0,0])
}

/// 功能: 获取系统中的的当前时间
/// 返回值: 是否执行成功，成功则返回 0
/// syscall ID: 169
pub fn sys_get_time() -> isize{
    syscall(SYSCALL_GET_TIME, [0,0,0])
}


pub fn sys_get_task_info(ti: &TaskInfo) -> isize {
    syscall(SYSCALL_TASK_INFO, [0, 0, 0])
}
