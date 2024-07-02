# src/os/task/switch.S

# 与 Trap 不同，他不涉及特权级切换
# 与 Trap 不同，他的一部分由编译器帮忙完成
# 与 Trap 不同，他对应用是透明的

.altmacro
.macro SAVE_SN n
	sd s\n, (\n+2)*8(a0)
.endm

.macro LOAD_SN n
	ld s\n, (\n+2)*8(a1)
.endm
	.section .text
	.globl _switch
_switch:
	# 阶段1 
	# _switch(
	#        current_task_cx_ptr: *mut TaskContent
	#        next_task_ptr: *const TaskContent
	#)
	# 阶段 2
	# save kernel stack of current task
	# A 在 A 任务在的上下文空间里保存 CPU 当前的寄存器快照
	sd sp, 8(a0)
	# save ra & s0 - s11 of current execution
	sd ra, 0(a0)
	.set n, 0
	.rept 12
		SAVE_SN %n 
		.set n, n+1
	.endr

	# 阶段 3
	# restore ra & s0 - s11 of next execution
	# 读取 next_task_ptr 指向的任务 B 任务的上下文，
	# 根据 B 任务的上下文保存的内容来恢复 ra，[s0--s11], sp 寄存器
	ld ra, 0(a1)
	.set n, 0
	.rept 12
		LOAD_SN %n 
		.set n, n+1 
	.endr
	# restore kernel stack of next task
	ld sp, 8(a1)
	# 阶段 4 
	# 完成 _switch 的返回，任务 B 可以从调用 _switch 的位置继续向下执行
	ret
