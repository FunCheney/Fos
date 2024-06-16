use core::arch::asm;

pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;

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
            //汇编 代码
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

/// 功能： 将内存中缓冲区中的数据写入文件
/// 参数： 'fd' 表示要写入的文件描述符
///        'buf' 表示内存中缓冲区的起始地址
///        'len' 表示内存中缓冲区的长度
/// 返回值： 返回成功写入的长度
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd,buffer.as_ptr() as usize, buffer.len()])
}


/// 功能: 退出应用程序并将返回值告知批处理系统
/// 参数: 'exit_code' 表示应用程序的返回值 
/// 返回值: 该系统调用不应该返回
pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}
