# Calling Convention
# Demo to create a leaf routine
#
# void _start()
# {
#     // calling leaf routine
#     square(3);
# }
#
# int square(int num)
# {
#     return num * num;
# }	

	.text
	.global _start
_start:
	la sp, stack_end

	li a0, 3
	call square

stop:
	j stop

square:
	# prologue
	addi sp, sp, -8
	sw s0, 0(sp)
	sw s1, 4(sp)

	
	mv s0, a0
	mul s1, s0, s0
	mv a0, s1
	
	# epilogue
	lw s0, 0(sp)
	lw s1, 4(sp)
	addi sp, sp, 8

	ret

stack_start:
	.rept 12
	.word 0
	.endr

stack_end:

	.end
