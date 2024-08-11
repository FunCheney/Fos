/// trap 管理

/// 首先是具体实现 Trap 上下文保存和恢复的汇编代码。
/// 在批处理操作系统初始化时，我们需要修改 stvec 寄存器来指向正确的 Trap 处理入口点。
mod context;

use crate::task::{exit_current_run_next, update_task_syscall_times};
// use crate::batch::run_next_app;
//use crate::{syscall::syscall, task::{exit_current_run_next, suspend_current_and_run_next}};
use crate::{syscall::syscall, task::suspend_current_and_run_next};
use crate::timer::set_next_trigger;
use core::arch::global_asm;
use log::{debug, error, info};
use riscv::register::sie;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    stval, stvec,
};

// 引入了一个外部符号 __alltraps ，并将 stvec 设置为 Direct 模式指向它的地址
// 在 os/src/trap/trap.S 中实现 Trap 上下文保存/恢复的汇编代码，分别用外部符号
//  __alltraps 和 __restore 标记为函数，并通过 global_asm! 宏将 trap.S 这段汇编代码插入进来。
// 引入 trap.S

global_asm!(include_str!("trap.S"));

pub fn init() {
    debug!("trap init call _alltraps in trap.S");
    extern "C" {
        fn _alltraps();
    }

    unsafe {
        stvec::write(_alltraps as usize, TrapMode::Direct);
    }
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}
    
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    // 进入用户态的时候，可以统计用户态的运行时间
    crate::task::user_time_end();

    let scause = scause::read();
    let stval = stval::read();
    // 根据 scause 寄存器所保存的 Trap 原因进行分发处理
    match scause.cause() {
        // 发现触发 Trap 的原因是来自 U 特权级的 Environment Call，也就是系统调用
        Trap::Exception(Exception::UserEnvCall) => {
            let syscall_id = cx.x[17];
            update_task_syscall_times(syscall_id);
            // 首先修改保存在内核栈上的 Trap 上下文里面 sepc，让其增加 4
            // 因为我们知道这是一个由 ecall 指令触发的系统调用，在进入 Trap 的时候，
            // 硬件会将 sepc 设置为这条 ecall 指令所在的地址（因为它是进入 Trap 之前最后一条执行的指令）。
            // 而在 Trap 返回之后，我们希望应用程序控制流从 ecall 的下一条指令 开始执行。
            // 因此我们只需修改 Trap 上下文里面的 sepc，让它增加 ecall 指令的码长，也即 4 字节。
            // 这样在 __restore 的时候 sepc 在恢复之后就会指向 ecall 的下一条指令，并在 sret 之后从那里开始执行。
            cx.sepc += 4;
            // Trap 上下文取出作为 syscall ID 的 a7 和系统调用的三个参数 a0~a2 传给 syscall 函数并获取返回值。
            // syscall 函数是在 syscall 子模块中实现的。 这段代码是处理正常系统调用的控制逻辑。
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }

        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            info!("[Kernel] PageFault in app, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, cx.sepc);
            exit_current_run_next();
            //run_next_app();
            //panic!("[kernel] not continue!");
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            info!("[kernel] IllegalInstruction in application.");
            //panic!("[kernel] not continue!");
            //run_next_app();
            exit_current_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            debug!("[kernel] SupervisorTimer in application.");
            //suspend_current_and_run_next();
            //panic!("[kernel] not continue!");
            // 当触发一个 S 特权级时钟中断，首先重置计时器
            set_next_trigger();
            // 调用函数
            suspend_current_and_run_next();
        }
        _ => {
            error!("[kernel] error in trap mod");
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    crate::task::user_time_start();
    cx
}

pub use context::TrapContext;
