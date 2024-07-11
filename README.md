###


### 第三章

[x] 相交于第二章的trap.S文件少了 mv sp, a0 

____restore在这里被两种情况复用了：
1.  正常从_alltraps走下来的trap_handler流程。如果是这种情况，trap_handler会在a0里返回之前通过mv a0, sp传进去的&mut TrapContext，所以这里sp和a0相同没有必要再mv sp, a0重新设置一遍。_
2. app第一次被__switch的时候通过__restore开始运行。这时候a0是个无关的数据（指向上一个TaskContext的指针），这里再mv sp a0就不对了，
而_restore要的TrapContext已经在__switch的恢复过程中被放在sp上了。（这个sp就是初始化时写完TrapContext后的内核栈顶）
```
for (i, task) in tasks.iter_mut().enumerate() {
    task.task_cx = TaskContext::goto_restore(init_app_cx(i));
    task.task_status = TaskStatus::Ready;
}
```
```
pub fn init_app_cx(app_id: usize) -> usize {
    KERNEL_STACK[app_id].push_context(TrapContext::app_init_context(
        get_base_i(app_id),
        USER_STACK[app_id].get_sp(),
    ))
}
```
```
impl TaskContext {
    /// set task context {__restore ASM funciton, kernel stack, s_0..12 }
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
```
