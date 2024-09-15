/// trap 管理

/// 首先是具体实现 Trap 上下文保存和恢复的汇编代码。
/// 在批处理操作系统初始化时，我们需要修改 stvec 寄存器来指向正确的 Trap 处理入口点。
mod context;

use crate::config::{TRAMPOLINE, TRAP_CONTEXT};
use crate::task::{current_trap_cx, current_user_token, exit_current_and_run_next};
use crate::timer::set_next_trigger;
use crate::{syscall::syscall, task::suspend_current_and_run_next};
use core::arch::{asm, global_asm};
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
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry() {
    // 一旦进入内核后再次触发到 S态 Trap，则硬件在设置一些 CSR 寄存器之后，会跳过对通用寄存器的保存
    // 过程，直接跳转到 trap_from_kernel 函数，在这里直接 panic 退出。这是因为内核和应用的地址空间
    // 分离之后，U态 –> S态 与 S态 –> S态 的 Trap 上下文保存与恢复实现方式/Trap 处理逻辑有很大差别。
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    // 把 stvec 设置为内核和应用地址空间共享的跳板页面的起始地址 TRAMPOLINE 而不是编译器在链接时看
    // 到的 __alltraps 的地址。这是因为启用分页模式之后，内核只能通过跳板页面上的虚拟地址来实际取得
    // __alltraps 和 __restore 的汇编代码。
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler() -> ! {
    // set_kernel_trap_entry 将 stvec 修改为同模块下另一个函数 trap_from_kernel 的地址
    set_kernel_trap_entry();
    let cx = current_trap_cx();
    // 进入用户态的时候，可以统计用户态的运行时间
    let scause = scause::read();
    let stval = stval::read();
    // 根据 scause 寄存器所保存的 Trap 原因进行分发处理
    match scause.cause() {
        // 发现触发 Trap 的原因是来自 U 特权级的 Environment Call，也就是系统调用
        Trap::Exception(Exception::UserEnvCall) => {
            let mut cx = current_trap_cx();
            // 首先修改保存在内核栈上的 Trap 上下文里面 sepc，让其增加 4
            // 因为我们知道这是一个由 ecall 指令触发的系统调用，在进入 Trap 的时候，
            // 硬件会将 sepc 设置为这条 ecall 指令所在的地址（因为它是进入 Trap 之前最后一条执行的指令）。
            // 而在 Trap 返回之后，我们希望应用程序控制流从 ecall 的下一条指令 开始执行。
            // 因此我们只需修改 Trap 上下文里面的 sepc，让它增加 ecall 指令的码长，也即 4 字节。
            // 这样在 __restore 的时候 sepc 在恢复之后就会指向 ecall 的下一条指令，并在 sret 之后从那里开始执行。
            cx.sepc += 4;
            cx = current_trap_cx();
            // Trap 上下文取出作为 syscall ID 的 a7 和系统调用的三个参数 a0~a2 传给 syscall 函数并获取返回值。
            // syscall 函数是在 syscall 子模块中实现的。 这段代码是处理正常系统调用的控制逻辑。
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }

        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::InstructionPageFault)
        | Trap::Exception(Exception::InstructionFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            info!("[Kernel] PageFault in app, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, cx.sepc);
            //run_next_app();
            exit_current_and_run_next(-2);
            //panic!("[kernel] not continue!");
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            info!("[kernel] IllegalInstruction in application.");
            //panic!("[kernel] not continue!");
            //run_next_app();
            exit_current_and_run_next(-3)
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
    trap_return();
}

#[no_mangle]
pub fn trap_return() -> ! {
    // set_user_trap_entry ，来让应用 Trap 到 S 的时候可以跳转到 __alltraps 。
    // 注：我们把 stvec 设置为内核和应用地址空间共享的跳板页面的起始地址
    set_user_trap_entry();
    // 准备好 __restore 需要两个参数：分别是 Trap 上下文在应用地址空间中的虚拟地址和要继续执行的应用地址空间的 token
    // 最后我们需要跳转到 __restore ，以执行：切换到应用地址空间、从 Trap 上下文中恢复通用寄存器、 sret 继续执行应用。
    // 它的关键在于如何找到 __restore 在内核/应用地址空间中共同的虚拟地址。
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_trap = current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    // 计算 __restore 虚地址的过程：由于 __alltraps 是对齐到地址空间跳板页面的起始地址 TRAMPOLINE 上
    // 的， 则 __restore 的虚拟地址只需在 TRAMPOLINE 基础上加上 __restore 相对于 __alltraps 的偏移
    // 量即可。这里 __alltraps 和 __restore 都是指编译器在链接时看到的内核内存布局中的地址。
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            // 首先需要使用 fence.i 指令清空指令缓存 i-cache 。这是因为，在内核中进行的一些操作可能
            // 导致一些原先存放某个应用代码的物理页帧如今用来存放数据或者是其他应用的代码，i-cache
            // 中可能还保存着该物理页帧的错误快照。因此我们直接将整个 i-cache 清空避免错误。接着使
            // 用 jr 指令完成了跳转到 __restore 的任务
            "fence.i",
            "jr {restore_va}",     // jump to new addr of _restore asm function
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,     // a0 = virt addr of trap Context
            in("a1") user_trap,       // a1 = phy addr of user page table
            options(noreturn)
        );
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel");
}

pub use context::TrapContext;
