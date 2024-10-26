use riscv::register::sstatus::{self, Sstatus, SPP};

/// TrapContext
/// Trap 发生调用时，需要保存物理资源内容，统一放在 TrapContext 中
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapContext {
    /// general regs [0:31]
    pub x: [usize; 32],
    /// CSR sstatus
    pub sstatus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
    // 内核地址空间的 token，内核页表的起始物理地址
    pub kernel_satp: usize,
    // 当前应用在内核地址空间中的内核栈栈顶的虚拟地址
    pub kernel_sp: usize,
    // 内核中 trap_handler 入口的虚拟地址
    pub trap_handler: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn app_init_context(
        entry: usize,
        sp: usize,
        kernel_satp: usize,
        kernel_sp: usize,
        trap_handler: usize,
    ) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,
            kernel_satp,
            kernel_sp,
            trap_handler,
        };
        cx.set_sp(sp);
        cx
    }
}
