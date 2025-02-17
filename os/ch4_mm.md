# 地址空间

地址空间是操作系统对物理内存的抽象，是运行的程序看到的系统中的内存。

一个进程的地址空间包含运行的程序的所有内存状态。比如：程序的代码（code，指令） 必须在内存中，因此它们在地址空间里。当程序在运行的时候，
利用栈（stack）来保存当 前的函数调用信息，分配空间给局部变量，传递参数和函数返回值。堆（heap）用于管理动态分配的、用户管理的内存，就像 java
中调用 new 获得内存。

当我们描述地址空间时，所描述的是操作系统提供给运行程序的抽象（abstract）。程序被加载到任意的物理地址，那么问题来了：

Q: 操作系统如何在单一的物理内存上为多个运行程序（所有进程共享内存）构建一个私有的，可能很大的地址空间抽象？
A: 虚拟化内存。应用程序拥有相同的起始地址(虚拟地址)，执行加载操作时，操作系统在硬件的支持下，加载到对应的物理地址，这是内存虚拟化的关键，这是
世界上每一个现代计算机系统的基础。

# 如何虚拟化内存

操作系统不仅虚拟化内存， 还有一定的风格。为了确保操作系统这样做，我们需要一些目标来指导。

虚拟内存（VM）系统的一个主要目标是透明（transparency）。操作系统实现虚拟内 存的方式，应该让运行的程序看不见。因此，程序不应该感知到内存被虚拟
化的事实，相反，程序的行为就好像它拥有自己的私有物理内存。在幕后，操作系统（和硬件）完成了所有的工作，让不同的工作复用内存，从而实现这个假象。

虚拟内存的另一个目标是效率（efficiency）。操作系统应该追求虚拟化尽可能高效 （efficient），包括时间上（即不会使程序运行得更慢）和空间
上（即不需要太多额外的内存来支持虚拟化）。在实现高效率虚拟化时，操作系统将不得不依靠硬件支持，包括 TLB 这样的硬件功能。

虚拟内存第三个目标是保护（protection）。操作系统应确保进程受到保护（protect）， 不会受其他进程影响，操作系统本身也不会受进程影响。当一个进程
执行加载、存储或指 令提取时，它不应该以任何方式访问或影响任何其他进程或操作系统本身的内存内容（即 在它的地址空间之外的任何内容）。因此，保护让
我们能够在进程之间提供隔离（isolation）的特性，每个进程都应该在自己的独立环境中运行，避免其他出错或恶意进程的影响。

# 虚拟内存

虚拟内存系统负责为程序提供一个巨大的、稀疏的、私有的地址空间的假象，其中保存了程序的所有指令和数据。操作系统在专门硬件的帮助下，通过每一个虚拟
内存的索引，将其转换为物理地址，物理内存根据获得的物理地址但获取所需的信息。操作系统会同时对许多进程执行此操作，并且确保程序之间互相不会受到影响，
也不会影响操作系统。整个方法需要大量的机制（很多底层机制）和一些关键的策略。

为了实现高效的虚拟化，操作系统应该尽量让程序自己运行， 同时通过在关键点的及时介入（interposing），来保持对硬件的控制。高效和控制是现代操作系统
的两个主要目标。

在实现虚拟内存时，实现高效和控制的同时，提供期望的虚拟化。

1. 高效决定了我们要利用硬件的支持，这在开始的时候非常初级（如使用一些寄存器），但会变得相当复杂（比如我们会讲到的 TLB、页表等）
2. 控制意味着操作系统要确保应用程序只能访问它自己的内存空间
3. 要保护应用程序不会相互影响，也不会影响操作系统，我们需要硬件的帮助

最后，我们对虚拟内存还有一点要求，即灵活性。具体来说，我们希望程序能以任何方式访问它自己的地址空间，从而让系统更容易编程。

Q: 如何实现高效的内存虚拟化？如何提供应用程序所需的灵活性？如何保持控制应用程序可访问的内存位置，从而确保应用程序的内存访问受到合理的限制？
如何高效地实现这一切？

A: 基于硬件的地址转换（hardware-based address translation），简称为地址转换（address translation）

# 地址转换

硬件对每次内存访问进行处理（即指令获取、数据读取或写 入），将指令中的虚拟（virtual）地址转换为数据实际存储的物理（physical）地址。因此，
在每次内存引用时，硬件都会进行地址转换，将应用程序的内存引用重定位到内存中实际的位置。

仅仅依靠硬件不足以实现虚拟内存，因为它只是提供了底层机制来提高效率。 操作系统必须在关键的位置介入，设置好硬件，以便完成正确的地址转换。因此它必
须管理内存（manage memory），记录被占用和空闲的内存位置，并明智而谨慎地介入，保持对内存使用的控制。

在虚拟内存中， 硬件可以介入到每次内存访问中，将进程提供的虚拟地址转换为数据实际存储的物理地址.

## 静态重定位

在硬件支持重定位之前，一些系统曾经采用纯软件的重定位方式。基本技术被称为静态重定位（static relocation），其中一个名为加载程序（loader）的软
件接手将要运行的可执行程序，将它的地址重写到物理内存中期望的偏移位置。

静态重定位有许多问题，首先也是最重要的是不提供访问保护，进程中的错误地址可能导致对其他进程或操作系统内存的非法访问，一般来说，需要硬件支持来实现
真正的访问保护。另一个缺点是一旦完成，稍后很难将内存空间重定位到其他位置。

## 动态重定位

每个 CPU 需要两个硬件寄存器：基址（base）寄存器和界限（bound）寄存器，有时称为限制（limit）寄存器。这组基址和界限寄存器，让我们能够将地址空
间放在物理内存的任何位置，同时又能确保进程只能访问自己的地址空间。

采用这种方式，在编写和编译程序时假设地址空间从零开始。但是，当程序真正执行时，操作系统会决定其在物理内存中的实际加载地址，并将起始地址记录在基址
寄存器中。

当进程运行时，该进程产生的所有内存引用，都会被处理器通过以下方式转换为物理地址：
```
physical address = virtual address + base
```
进程中使用的内存引用都是虚拟地址（virtual address），硬件接下来将虚拟地址加上基址寄存器中的内容，得到物理地址（physical address），再发给
内存系统。

将虚拟地址转换为物理地址，这正是所谓的地址转换（address translation）技术。也就是说，硬件取得进程认为它要访问的地址，将它转换成数据实际位于
的物理地址。由于这种重定位是在运行时发生的，而且我们甚至可以在进程开始运行后改变其地址空间，这种技术一般被称为动态重定位。

在动态重定位的过程中，只有很少的硬件参与，但获得了很好的效果。一个基址寄存器将虚拟地址转换为物理地址，一个界限寄存器确保这个地址在进程地址空间的
范围内。它们一起提供了既简单又高效的虚拟内存机制。

关于界限寄存器，它通常有两种使用方式。在一种方式中（像上面那样），它记录地址空间的大小，硬件在将虚拟地址与基址寄存器内容求和前，就检查这个界限。
另一种方式是界限寄存器中记录地址空间结束的物理地址，硬件在转化虚拟地址到物理地址之后才去检查这个界限。这两种方式在逻辑上是等价的。


## 硬件支持：总结
1. 两种 CPU 模式。操作系统在特权模式（privileged mode，或内核模式，kernel mode），可以访问整个机器资源。应用程序在用户模式（user mode）
   运行，只能做有限的操作。只要一个位，也许保存在处理器状态字（processor status word）中，就能说明当前的 CPU 运行模式。在一些特殊的时刻
   （如系统调用、异常或中断），CPU 会切换状态
2. 硬件还必须提供基址和界限寄存器（base and bounds register），因此每个 CPU 的内存管理单元（Memory Management Unit，MMU）都需要这两个
   额外的寄存器。用户程序运行时，硬件会转换每个地址，即将用户程序产生的虚拟地址加上基址寄存器的内容。硬件也必须能检查地址是否有用，通过界限寄存
   器和 CPU 内的一些电路来实现
3. 硬件应该提供一些特殊的指令，用于修改基址寄存器和界限寄存器，允许操作系统在切换进程时改变它们。这些指令是特权（privileged）指令，只有在内核
   模式下，才能修改这些寄存器
4. 在用户程序尝试非法访问内存（越界访问）时，CPU必须能够产生异常（exception）。 在这种情况下，CPU 应该阻止用户程序的执行，并安排操作系统的“越
   界”异常处理程序 （exception handler）去处理。操作系统的处理程序会做出正确的响应，比如在这种情况下终止进程。类似地，如果用户程序尝试修改基
   址或者界限寄存器时，CPU 也应该产生异常，并调用“用户模式尝试执行特权指令”的异常处理程序。CPU 还必须提供一种方法，来通知它这些处理程序的位置，
   因此又需要另一些特权指令。

## 软件支持（操作系统）：总结
1. 在进程创建时，操作系统必须采取行动，为进程的地址空间找到内存空间
2. 在进程终止时（正常退出，或因行为不端被强制终止），操作系统也必须做一些工作，回收它的所有内存，给其他进程或者操作系统使用
3. 在上下文切换时，操作系统也必须执行一些额外的操作。每个 CPU 毕竟只有一 个基址寄存器和一个界限寄存器，但对于每个运行的程序，它们的值都不同，
   因为每个程序被加载到内存中不同的物理地址。因此，在切换进程时，操作系统必须保存和恢复基础和界限寄存器。具体来说，当操作系统决定中止当前的运行
   进程时，它必须将当前基址和界限寄存器中的内容保存在内存中，放在某种每个进程都有的结构中，如进程结构（process structure）或进程控制块
  （Process Control Block，PCB）中。类似地，当操作系统恢复执行某个进程时（或第一次执行），也必须给基址和界限寄存器设置正确的值。
4. 操作系统必须提供异常处理程序（exception handler），或要一些调用的函数，像上面提到的那样。操作系统在启动时加载这些处理程序（通过特权命令）
   当一个进程试图越界访问内存时，CPU 会触发异常。在这种异常产生时，操作系统必须准备采取行动。通常操作系统会做出充满敌意的反应：终止错误进程。

## 带来的问题
我们一直假设将所有进程的地址空间完整地加载到内存中。利用基址和界限寄存器，操作系统很容易将不同进程重定位到不同的物理内存区域。
1. 将整个地址空间放入物理内存，那么栈和堆之间的空间并没有被进程使用，却依然占用了实际的物理内存。因此，简单的通过基址寄存器和界限寄存器实现的虚
   拟内存很浪费。
2. 如果剩余物理内存无法提供连续区域来放置完整的地址空间，进程便无法运行。

Q：怎样支持大地址空间，同时栈和堆之间（可能）有大量空闲空间？
A：分段！

# 分段
在 MMU 中引入不止一个基址和界限寄存器对，而是给地址空间内的每个逻辑段（segment）一对。一个段只是地址空间里的一个连续定长的区域。

在典型的地址空间里有 3 个逻辑不同的段：代码、栈和堆。分段的机制使得操作系统能够将不同的段放到不同的物理内存区域，从而避免了虚拟地址空间中的未使
用部分占用物理内存。

硬件在地址转换的时候使用段寄存器。
每个段都有自己的名字。为了实现简单起见，通常可用一个段号来代替段名，每个段都从 0 开始编址，并采用一段连续的地址空间。段的长度由相应的逻辑信息组
的长度决定，因而各段长度不等。整个作业的地址空间由于是分成多个段，因而是二维的，亦即，其逻辑地址由段号(段名)和段内地址所组成。

```
 ------------------------------
 |     段号      |   段内地址   |
31 -----------16 15-----------0
```
在该地址结构中，允许一个作业最长有 64 K 个段，每个段的最大长度为 64 KB。
分段方式已得到许多编译程序的支持，编译程序能自动地根据源程序的情况而产生若干个段。

## 段表
在分段式存储管理系统中，则是为每个分段分配一个连续的分区，而进程中的各个段可以离散地移入内存中不同的分区中。能从物理内存中找出每个
逻辑段所对应的位置。
在系统中为每个进程建立一张段映射表，简称“段表”。每个段在表中占有一个表项，其中记录了该段在内存中的起始地址(又称为“基址”)和段的长度。段表可以存
放在一组寄存器中，这样有利于提高地址转换速度，但更常见的是将段表放在内存中。
在配置了段表后，执行中的进程可通过查找段表找到每个段所对应的内存区。可见， 段表是用于实现从逻辑段到物理内存区的映射。

## 地址转换
为了实现从进程的逻辑地址到物理地址的变换功能，在系统中设置了段表寄存器，用于存放段表始址和段表长度 TL。在进行地址变换时，系统将逻辑地址中的段号
与段表长度 TL 进行比较。若 S>TL，表示段号太大，是访问越界，于是产生越界中断信号；若未越界，则根据段表的始址和该段的段号，计算出该段对应段表项
的位置，从中读出该段在内存的起始地址，然后，再检查段内地址 d 是否超过该段的段长 SL。若超过，即 d>SL，同样发出越界中断信号；若未越界，则将该段
的基址 d 与段内地址相加，即可得到要访问的内存物理地址。

## 操作系统的支持
1. 操作系统在上下文切换时应该做什么？ 
   各个段寄存器中的内容必须保存和恢复。显然，每个进程都有自己独立的虚拟地址空间，操作系统必须在进程运行前，确保这些寄存器被正确地赋值。
2. 外部碎片
   新的地址空间被创建时，操作系统需要在物理内存中为它的段找到空间。之前，我们假设所有的地址空间大小相同，物理内存可以被认为是一些槽块，进程可以
   放进去。现在，每个进程都有一些段，每个段的大小也可能不同。一般会遇到的问题是，物理内存很快充满了许多空闲空间的小洞，因而很难分配给新的段，
   或扩大已有的段。这种问题被称为外部碎片。
   a. 该问题的一种解决方案是紧凑（compact）物理内存，重新安排原有的段。
   b. 一种更简单的做法是利用空闲列表管理算法,试图保留大的内存块用于分配。

# 分页

分页不是将一个进程的地址空间分割成几个不同长度的逻辑段（即代码、堆、段），而是分割成固定大小的单元，每个单元称为一页。相应地，我们把物理内存看
成是定长槽块的阵列，叫作页帧（page frame）。每个这样的页帧包含一个虚拟内存页。

Q：如何通过页来实现虚拟内存，从而避免分段的问题？基本技术是什么？如何让这些技术运行良好， 并尽可能减少空间和时间开销？

### 页表

#### 地址转换

#### 页表

#### 快表

#### 二级页表与多级页表







































