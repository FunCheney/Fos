//! batch subsystem

use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use core::arch::asm;
use lazy_static::*;

/// 指定用户栈的大小
const USER_STACK_SIZE: usize = 4096 * 2;
/// 指定内核栈的大小 
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

/// 内核栈
#[repr(align(4096))]
struct KernelStack{
    data: [u8; KERNEL_STACK_SIZE],
}

/// 用户栈
#[repr(align(4096))]
struct UserStack{
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE]
};
static USER_STACK: UserStack = UserStack{
    data: [0; USER_STACK_SIZE]
};

impl KernelStack {
    /// 获取栈顶的位置，在 risc-v 中，栈向下生长
    fn get_sp(&self) -> usize {
        // 返回数结尾的地址
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
    
}

struct AppManager {
    num_app: usize,
    // 表示当前执行的是第几个应用
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self){
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x}, {:#x}",
                i,
                self.app_start[i],
                self.app_start[i + 1]
                );
        }
    }

    /// 该方法负责将 app_id 对应的应用程序二进制文件加载到
    /// 物理内存 0x80400000 对应的位置
    unsafe fn load_app(&self, app_id: usize){
        if app_id > self.num_app {
            println!("all application completed");
            shutdown(false);
        }

        println!("[kernel] loading app_{}", app_id);

        // 首先清理一块内存地址
        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        // 找到 待加载应用二进制镜像的位置。本质就是把数据从一块内存复制到另一块内存
        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );

        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);

        // 加载完后，插入汇编指令，它的作用是: 在它之后取址过程必须能够看到在它之前的所有对取址区域
        // 的修改
        asm!("fence.i");
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }
    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }
}

// lazy_static 宏，提供了全局变量初始化的功能
// 一般情况下，全局变量必须在编译期设置初始值， 但是有些全局变量的初始化依赖于运行期间才能得到的数据。 
// 如这里我们借助 lazy_static! 声明了一个 AppManager 结构的名为 APP_MANAGER 的全局实例， 
// 只有在它第一次被使用到的时候才会进行实际的初始化工作。

lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                // 找到 link_app.S 中的符号 _num_app 
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

/// init batch subsystem
pub fn init() {
    print_app_info();
}

/// print apps info
pub fn print_app_info() {
    // 会用到 APP_MANAGER，也是在第一次调用的时候初始化
    APP_MANAGER.exclusive_access().print_app_info();
}

/// run next app
pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    unsafe {
        app_manager.load_app(current_app);
    }
    app_manager.move_to_next_app();
    drop(app_manager);
    // before this we have to drop local variables related to resources manually
    // and release the resources
    extern "C" {
        fn _restore(cx_addr: usize);
    }
    unsafe {
        // _restore 所做的事情是在内核栈上压入一个 Trap 上下文，
        // 其 sepc 是应用程序入口地址 0x80400000 ，其 sp 寄存器指向用户栈，其 sstatus 的 SPP 字段被设置为 User 。
        // push_context 的返回值是内核栈压入 Trap 上下文之后的栈顶，它会被作为__restore 的参数（ 回看 __restore 代码 ，这时我们可以理解为何 __restore 函数的起始部分会完成 ），
        // 这使得在 _restore 函数中 sp 仍然可以指向内核栈的栈顶。这之后，就和执行一次普通的 _restore 函数调用一样了。 
        _restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }
    panic!("Unreachable in batch::run_current_app!");
}
