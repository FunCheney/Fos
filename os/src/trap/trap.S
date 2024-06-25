# 首先通过 __alltraps 将 Trap 上下文保存在内核栈上，
# 然后跳转到使用 Rust 编写的 trap_handler 函数 完成 Trap 分发及处理。
# 当 trap_handler 返回之后，使用 __restore 从保存在内核栈上的 Trap 上下文恢复寄存器。
# 最后通过一条 sret 指令回到应用程序执行。

.altmacro  # 加上 .altmacro 才能正常使用 .rept 命令。
.macro SAVE_GP n
	sd x\n, \n*8(sp)
.endm
.macro LOAD_GP n
	ld x\n, \n*8(sp)
.endm

	.section .text 
	.global _alltraps
	.global _restore
	.align 2  # 将_alltraps 按照 4 字节对齐

_alltraps:
	csrrw sp, sscratch, sp  # 将 CSR 当前集训器的值读到通用寄存器 rd 中，
				# 然后将通用寄存器 rs 的值写入到 CSR 中。
				# 因此这里起到的是交换 sscratch 和 sp 的效果。
				# 在这一行之前 sp 指向用户栈， sscratch 指向内核栈
				# 现在 sp 指向内核栈， sscratch 指向用户栈。
	# now sp -> kerner stack, sscrath -> user stack
	# allocate a TrapContext on kerner stack
	# 在内核栈上保存 Trap 上下文，于是预先分配 34 * 8 字节的栈帧，
	# 这里改动的是 sp ，说明确实是在内核栈上。
	addi sp, sp, -34*8
	# save general-purpose registers
	# 跳过 x0 寄存器
	sd x1, 1*8(sp)
	# skip sp(x2), will save it later
	# 跳过 x2 寄存器，x2 指向内核栈
	sd x3, 3*8(sp)
	# skip tp(x4), application does not use it
        # save x5~x31
	.set n, 5  # 5~x31 这 27 个通用寄存器我们通过类似循环的 .rept 每次使用 SAVE_GP 宏来保存实质是相同的
	.rept 27
	     SAVE_GP %n
	     .set n, n+1
	.endr
	# we can use t0/t1/t2 freely, because they were saved on kernel stack
	csrr t0, sstatus # 将 CSR sstatus 保存在 t0 上
	csrr t1, sepc    # 将 CSR spec 保存在 t1 上
	sd t0, 32*8(sp)
	sd t1, 33*8(sp)
	# read user stack from sscratch and save it on the kernel stack
	csrr t2, sscratch  # 首先将 sscratch 的值读到寄存器 t2 并保存到内核栈上
	sd t2, 2*8(sp)
        # set input argument of trap_handler(cx: &mut TrapContext)
	# 寄存器 a0 指向内核栈的栈指针保存的就是 Trap 上下文的地址，
	# 接下来要调用 trap_handler 进行 Trap 处理，它的第一个参数 cx 由调用规范要从 a0 中获取
        mv a0, sp
        call trap_handler

_restore:
	# case1: start running app by __restore
	# case2: back to U after handling trap
	mv sp, a0
	# now sp-> kernel stack[after allocate] sscrath -> User stack
	# restore sstatus/sepc
	ld t0, 32*8(sp)
	ld t1, 33*8(sp)
	ld t2, 2*8(sp)
	csrw sstatus,t0
	csrw sepc,t1
	csrw sscratch, t2
	# restore general-purpose registers except sp/tp
	ld x1, 3*8(sp)
	ld x3, 3*8(sp)
	.set n, 5
	.rept 27
		LOAD_GP %n
		.set n, n+1
	.endr
	# release TrapContext on kernel stack
	addi sp, sp, 34*8
	# now sp->kernel stack, sscratch->user stack
	csrrw sp, sscratch, sp
	# sret 指令返回到 U 特权级继续运行应用程序控制流
	sret
