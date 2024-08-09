# 食用指南

## 第三章

* 相比较于第二章的trap.S文件少了 mv sp, a0

* _restore 在这里被两种情况复用了：

 1. 正常从_alltraps 走下来的 trap_handler 流程。如果是这种情况，trap_handler会在a0里返回之前通过mv a0, sp
传进去的 &mut TrapContext，所以这里sp和a0相同没有必要再mv sp, a0重新设置一遍

 2. app第一次被__switch的时候通过_restore开始运行。这时候a0是个无关的数据（指向上一个 TaskContext 的指针），
这里再mv sp a0就不对了，而 _restore 要的 TrapContext 已经在 __switch 的恢复过程中被放在 sp 上了。
（这个sp就是初始化时写完TrapContext后的内核栈顶）。

```Rust
for (i, task) in tasks.iter_mut().enumerate() {
    task.task_cx = TaskContext::goto_restore(init_app_cx(i));
    task.task_status = TaskStatus::Ready;
}
```

```Rust
pub fn init_app_cx(app_id: usize) -> usize {
    KERNEL_STACK[app_id].push_context(TrapContext::app_init_context(
        get_base_i(app_id),
        USER_STACK[app_id].get_sp(),
    ))
}
```

```Rust
impl TaskContext {
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn _restore();
        }
        Self {
            ra: _restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
```

#### 编程题

1. 扩展内核能够展示操作系统切换任务的过程



#### 任务的地中空间中有那些类型的数据和代码

* .text: 任务的代码段，其中开头的 .text.entry 段包含任务的入口地址
* **.rodata：只读数据，包含字符串常量，如测例中的 println!("Test power_3 OK!"); 实际打印的字符串存在这里
* .data：需要初始化的全局变量
* .bss：未初始化或初始为0的全局变量。

#### 协作式调度与抢占式调度的区别

协作式调度中，进程主动放弃（yield）执行资源，暂停运行，将占用的资源转让给其他进程。
抢占式调度中，进程会被强制打断暂停，释放资源给进程。

#### 中断，异常和系统调用有何异同之处

* 相同点：
    * 都会从通常的控制流中跳出，进入 trap_handler 中进行处理
* 不同点：
    * 中断的来源是异步的外部事件，由外设，时钟，别的 hart 等外部来源，与 CPU 正在做什么没关系
    * 异常是CPU 正在执行的指令遇到问题无法正常进行而产生的。
    * 系统调用是程序有意想让操作系统帮忙执行一些操作，用专门的指令（如 ecall）触发的。

