use riscv::register::sstatus::{self, Sstatus, SPP};

/// TrapContext
/// Trap 发生调用时，需要保存物理资源内容，统一放在 TrapContext 中
#[repr(C)]
pub struct TrapContext {

    /// general regs [0:31]
    pub x:[usize;32],
    /// CSR sstatus
    pub sstatus: Sstatus,
    /// CSR sepc
    pub sepc: usize,
} 

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }

    pub fn app_init_context(entry: usize, sp: usize) -> Self{
        let mut sstatus = sstatus::read(); // CSR sstatus
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x:[0;32],
            sstatus,
            sepc: entry,
        };
        cx.set_sp(sp);
        cx
    }
} 

