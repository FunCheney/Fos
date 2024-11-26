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



### 第四章

CPU  访问数据和内存的地址是虚地址，通过硬件机制（MMU + 页表查询）进行地址转换，找到对应的物理地址。
为此，计算机科学家提出了地址空间（Address space）的抽象，并在内核中建立虚拟地址空间映射的机制，给
应用程序提供一个基于地址空间的安全虚拟内存环境，让应用程序简单灵活的使用内存。

#### 虚拟地址与虚拟地址空间

到目前为止，仍被操作系统广泛使用的抽象被称为地址空间，某种程度上讲可以看做是一块巨大的但并不一定真实
存在的内存。在每个应用程序的视角里，操作系统分配给一个程序的地址范围受限（容量很大），独占的连续地址空间
因此，应用程序可以在划分给他的内存空间中随意规划内存布局。它的各个段也就可以放置在地址空间中他希望的位置。

应用同样可以使用一个地址作为索引来读写自己的地址空间的数据，就像使用物理地址作为索引来读取物理内存上的数据
一样。这种地址称为虚拟地址。

当然操作系统要达到地址空间抽象的设计目标，需要计算机硬件的支持，这就涉及到 MMU 和 TLB 等硬件机制。

每个应用独占一个地址空间，里面只包含有自己的各个段，于是他可以规划属于自己的各个段的分布而无须考虑和其他应用的冲突；
同时鉴于只能通过虚拟地址读写他自己的空间地址，他完全无法窃取或者破坏其他应用的数据，因为那些地址是他无法访问的。

这是地址空间抽象和具体硬件机制对应用程序的安全性和稳定性的一种保障。

 
#### 增加硬件加速虚实地址转换

当 CPU 取指令或者执行一条访存指令的时候，它都是基于虚拟地址访问属于当前正在运行的应用的地址空间。
此时，CPU 中的 内存管理单元 (MMU, Memory Management Unit) 自动将这个虚拟地址进行 地址转换 (Address Translation) 
变为一个物理地址，即这个应用的数据/指令的物理内存位置。
也就是说，在 MMU 的帮助下，应用对自己虚拟地址空间的读写才能被实际转化为对于物理内存的访问。

事实上，每个应用的地址空间都存在一个从虚拟地址到物理地址的映射关系。可以想象对于不同的应用来说，该映射可能是不同的，
即 MMU 可能会将来自不同两个应用地址空间的相同虚拟地址转换成不同的物理地址。要做到这一点，就需要硬件提供一些寄存器，
软件可以对它进行设置来控制 MMU 按照哪个应用的地址映射关系进行地址转换。于是，将应用的代码/数据放到物理内存并进行管理
，建立好应用的地址映射关系，在任务切换时控制 MMU 选用应用的地址映射关系，则是作为软件部分的内核需要完成的重要工作。

回过头来，在介绍内核对于 CPU 资源的抽象——时分复用的时候，我们曾经提到它为应用制造了一种每个应用独占整个 CPU 的幻象，
而隐藏了多个应用分时共享 CPU 的实质。而地址空间也是如此，应用只需、也只能看到它独占整个地址空间的幻象，
而藏在背后的实质仍然是多个应用共享物理内存，它们的数据分别存放在内存的不同位置。

地址空间只是一层抽象接口，它有很多种具体的实现策略。对于不同的实现策略来说，操作系统内核如何规划应用数据放在物理内存的位置，
而 MMU 又如何进行地址转换也都是不同的。下面我们简要介绍几种曾经被使用的策略，并探讨它们的优劣。


#### 分段内存管理
曾经的一种做法如上图所示：每个应用的地址空间大小限制为一个固定的常数 bound ，也即每个应用的可用虚拟地址区间均为[0. boung) 。
随后，就可以以这个大小为单位，将物理内存除了内核预留空间之外的部分划分为若干个大小相同的 插槽 (Slot) ，
每个应用的所有数据都被内核放置在其中一个插槽中，对应于物理内存上的一段连续物理地址区间，假设其起始物理地址为 base ，则由于二者大小相同，
这个区间实际为[base, base + bound) 。因此地址转换很容易完成，只需检查一下虚拟地址不超过地址空间的大小限制（此时需要借助特权级机制通过异常来进行处理），
然后做一个线性映射，将虚拟地址加上 base  就得到了数据实际所在的物理地址。

可以看出，这种实现极其简单：MMU 只需要 bound, base 两个寄存器，在地址转换进行比较或加法运算即可；而内核只需要在任务切换时完成切换 base寄存器。
在对一个应用的内存管理方面，只需考虑一组插槽的占用状态，可以用一个 位图 (Bitmap) 来表示，随着应用的新增和退出对应置位或清空。

然而，它的问题在于：可能浪费的内存资源过多。注意到应用地址空间预留了一部分，它是用来让栈得以向低地址增长，
同时允许堆往高地址增长（支持应用运行时进行动态内存分配）。每个应用的情况都不同，内核只能按照在它能力范围之内
的消耗内存最多的应用的情况来统一指定地址空间的大小，而其他内存需求较低的应用根本无法充分利用内核给他们分配的这部分空间。
但这部分空间又是一个完整的插槽的一部分，也不能再交给其他应用使用。这种在已分配/使用的地址空间内部无法被充分利用的空间就是 内碎片 (Internal Fragment) ，
它限制了系统同时共存的应用数目。如果应用的需求足够多样化，那么内核无论如何设置应用地址空间的大小限制也不能得到满意的结果。这就是固定参数的弊端：虽然实现简单，但不够灵活。

#### 分页内存管理