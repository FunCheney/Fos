use crate::SignalAction;
use core::arch::asm;

const SYSCALL_DUP: usize = 24;
const SYSCALL_OPEN: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_KILL: usize = 129;
const SYSCALL_SIGACTION: usize = 134;
const SYSCALL_SIGPROCMASK: usize = 135;
const SYSCALL_SIGRETURN: usize = 139;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;
const SYSCALL_TASK_INFO: usize = 410;

const SYSCALL_THREAD_CREATE: usize = 1000;
const SYSCALL_GETTID: usize = 1001;
const SYSCALL_WAITTID: usize = 1002;
const SYSCALL_MUTEX_CREATE: usize = 1010;
const SYSCALL_MUTEX_LOCK: usize = 1011;
const SYSCALL_MUTEX_UNLOCK: usize = 1012;
const SYSCALL_SEMAPHORE_CREATE: usize = 1020;
const SYSCALL_SEMAPHORE_UP: usize = 1021;
const SYSCALL_SEMAPHORE_DOWN: usize = 1022;
const SYSCALL_CONDVAR_CREATE: usize = 1030;
const SYSCALL_CONDVAR_SIGNAL: usize = 1031;
const SYSCALL_CONDVAR_WAIT: usize = 1032;
const SYSCALL_FRAMEBUFFER: usize = 2000;
const SYSCALL_FRAMEBUFFER_FLUSH: usize = 2001;
const SYSCALL_EVENT_GET: usize = 3000;
const SYSCALL_KEY_PRESSED: usize = 3001;

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

pub fn sys_dup(fd: usize) -> isize {
    syscall(SYSCALL_DUP, [fd, 0, 0])
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
/// 参数: path 给出了要加载的可执行文件的名
/// 参数: args 数组总每一个元素都是命令行字符串的起始地址
/// 返回值: 如果出错的话（如果找不到名字相符的可执行文件）则返回 -1, 否则不应该返回
/// syscall ID: 221
pub fn sys_exec(path: &str, args: &[*const u8]) -> isize {
    syscall(
        SYSCALL_EXEC,
        [path.as_ptr() as usize, args.as_ptr() as usize, 0],
    )
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

/// 功能：为当前进程打开一个管道。
/// 参数：pipe 表示应用地址空间中的一个长度为 2 的 usize 数组的起始地址，内核需要按顺序将管道读端
/// 和写端的文件描述符写入到数组中。
/// 返回值：如果出现了错误则返回 -1，否则返回 0 。可能的错误原因是：传入的地址不合法。
/// syscall ID：59
pub fn sys_pipe(pipe: &mut [usize]) -> isize {
    syscall(SYSCALL_PIPE, [pipe.as_mut_ptr() as usize, 0, 0])
}

pub fn sys_kill(pid: usize, signal: i32) -> isize {
    syscall(SYSCALL_KILL, [pid, signal as usize, 0])
}

pub fn sys_sigaction(
    signum: i32,
    action: *const SignalAction,
    old_action: *mut SignalAction,
) -> isize {
    syscall(
        SYSCALL_SIGACTION,
        [signum as usize, action as usize, old_action as usize],
    )
    /*
    syscall(
        SYSCALL_SIGACTION,
        [
            signum as usize,
            action.map_or(0, |r| r as *const _ as usize),
            old_action.map_or(0, |r| r as *mut _ as usize),
        ],
    )
    */
}

pub fn sys_sigprocmask(mask: u32) -> isize {
    syscall(SYSCALL_SIGPROCMASK, [mask as usize, 0, 0])
}

pub fn sys_sigreturn() -> isize {
    syscall(SYSCALL_SIGRETURN, [0, 0, 0])
}

/// 创建线程
pub fn sys_thread_create(enrty: usize, arg: usize) -> isize {
    syscall(SYSCALL_THREAD_CREATE, [enrty, arg, 0])
}

/// 获取线程id
pub fn sys_gettid() -> isize {
    syscall(SYSCALL_GETTID, [0, 0, 0])
}

/// 获取等待线程id
pub fn sys_waittid(tid: usize) -> isize {
    syscall(SYSCALL_WAITTID, [tid, 0, 0])
}
/// 功 能: 为 当 前 进 程 新 增 一 把 互 斥 锁
/// 参 数: blocking 为 true 表 示 互 斥 锁 基 于 阻 塞 机 制 实 现
/// 否 则 表 示 互 斥 锁 基 于 类 似 yield 的 方 法 实 现
/// 返 回 值: 假 设 该 操 作 必 定 成 功 ， 返 回 创 建 的 锁 的 ID
/// syscall ID: 1010
pub fn sys_mutex_create(blocking: bool) -> isize {
    syscall(SYSCALL_MUTEX_CREATE, [blocking as usize, 0, 0])
}

/// 功能: 当前线程尝试获取所属进程的一把互斥锁
/// 参数: id 表示要获取锁的id
/// 返回值: 0
/// syscall ID: 1011
pub fn sys_mutex_lock(id: usize) -> isize {
    syscall(SYSCALL_MUTEX_LOCK, [id, 0, 0])
}
/// 功能: 当前线程尝试释放所属进程的一把互斥锁
/// 参数: id 表示要释放锁的 ID
/// 返回值: 0
/// syscall Id: 1012
pub fn sys_mutex_unlock(id: usize) -> isize {
    syscall(SYSCALL_MUTEX_UNLOCK, [id, 0, 0])
}

/// 功能: 为当前进程新增一个信号量
/// 参数: res_count 表示该信号量的初始资源可用数量，即 N，为一个非负数
/// 返回值: 0
/// syscall ID: 1020
pub fn sys_semaphore_create(res_count: usize) -> isize {
    syscall(SYSCALL_SEMAPHORE_CREATE, [res_count, 0, 0])
}

/// 功能: 对当前进程中的信号量进行 V 操作 (归还)
/// 参数: sem_id 表示要进行 V 操作的信号量Id
/// 返回值: 假定操作必成功，返回 0
/// syscall ID: 1021
pub fn sys_semaphore_up(sem_id: usize) -> isize {
    syscall(SYSCALL_SEMAPHORE_UP, [sem_id, 0, 0])
}

/// 功能: 对当前进程中的信号量进行 P 操作 (尝试占用)
/// 参数: sem_id 表示要进行 V 操作的信号量Id
/// 返回值: 假定操作必成功.返回 0
/// syscall ID: 1022
pub fn sys_semaphore_down(sem_id: usize) -> isize {
    syscall(SYSCALL_SEMAPHORE_DOWN, [sem_id, 0, 0])
}

pub fn sys_condvar_create() -> isize {
    syscall(SYSCALL_CONDVAR_CREATE, [0, 0, 0])
}

pub fn sys_condvar_signal(condvar_id: usize) -> isize {
    syscall(SYSCALL_CONDVAR_SIGNAL, [condvar_id, 0, 0])
}

pub fn sys_condvar_wait(condvar_id: usize, mutex_id: usize) -> isize {
    syscall(SYSCALL_CONDVAR_WAIT, [condvar_id, mutex_id, 0])
}
