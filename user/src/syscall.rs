use core::arch::asm;

pub const SYSCALL_OPEN: usize = 56;
pub const SYSCALL_CLOSE: usize = 57;
pub const SYSCALL_READ: usize = 63;
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_YIELD: usize = 124;
pub const SYSCALL_GET_TIME: usize = 169;
pub const SYSCALL_TASK_INFO: usize = 410;
pub const SYSCALL_FORK: usize = 220;
pub const SYSCALL_GETPID: usize = 172;
pub const SYSCALL_EXEC: usize = 221;
pub const SYSCALL_WAITPID: usize = 260;

/// 功能: 将系统调用封装成 syscall 函数
/// 参数: 'id' 系统调用id
///       'args' 三个参数
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
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
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
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
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

/// 功能: 获取系统中的的当前时间
/// 返回值: 是否执行成功，成功则返回 0
/// syscall ID: 169
pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}

#[allow(unused)]
pub fn sys_get_task_info() -> isize {
    syscall(SYSCALL_TASK_INFO, [0, 0, 0])
}

/// 功能: 从当前进程 fork 出一个子进程来
/// 返回值: 对于子进程方会0, 对于当前进程返回子进程 PID
/// syscall ID: 220
pub fn sys_fork() -> isize {
    syscall(SYSCALL_FORK, [0, 0, 0])
}

/// 功能: 当前
pub fn sys_getpid() -> isize {
    syscall(SYSCALL_GETPID, [0, 0, 0])
}

/// 功能: 将当前进程的地址空间清空，并加载一个特定的可执行文件，返回用户态之后开始执行他
/// 参数: path 给出了要加载的可执行文件的名字
/// 返回值: 如果出错的话（如果找不到名字相符的可执行文件）则返回 -1, 否则不应该返回
/// syscall ID: 221
pub fn sys_exec(path: &str) -> isize {
    syscall(SYSCALL_EXEC, [path.as_ptr() as usize, 0, 0])
}

/// 功能: 当前进程等待一个子进程变为僵尸进程，回收其全部资源并收集其返回值
/// 参数: pid 表示要等待的子进程的进程id，如果返回 -1 表示等待任意一个子进程
/// exit_code 表示保存子进程返回值的地址，如果该值为 0 表示不必保存
/// 返回值: 如果等待的子进程不存在则返回 -1; 否则如果等待的子进程均未结束则返回 -2
/// 否则返回结束子进程的进程 pid
/// syscall id: 260
pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    syscall(SYSCALL_WAITPID, [pid as usize, exit_code as usize, 0])
}

/// 功能: 从文件中读出一段内容到缓冲区
/// 参数: fd 带读取文件的文件描述符，buffer 切片给出的缓冲区
/// 返回值: 如果出现错误返回 -1, 否则返回实际读到的字节数
/// syscall id: 63
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(
        SYSCALL_READ,
        [fd, buffer.as_mut_ptr() as usize, buffer.len()],
    )
}

/// 功能: 打开一个文件，并返回可以访问它的文件描述符
/// 参数: path 描述要打开的文件名（简单起见，文件系统不需要支持目录，所有文件都放在根目录下）
/// flags: 描述打开文件的标志
/// 返回值: 如果出现了错误则返回 -1，否则返回打开文件的文件描述符。可能的错误原因：文件不存在
/// syscall id: 56
pub fn sys_open(path: &str, flags: u32) -> isize {
    syscall(SYSCALL_OPEN, [path.as_ptr() as usize, flags as usize, 0])
}

/// 功能: 当前进程关闭一个文件
/// 参数: fd 表示当前文件的文件描述符
/// 返回值: 如果是 0 成功关闭，-1 关闭失败。可能错误的原因：传入的文件描述符不是一个对应的打开文件
/// syscall id: 57
pub fn sys_close(fd: usize) -> isize {
    syscall(SYSCALL_CLOSE, [fd, 0, 0])
}
