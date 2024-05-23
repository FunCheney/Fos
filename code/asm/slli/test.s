# Shift Left LOGICAL Immediate
# Format
#   SLLI RD, RS, IMM 
# Desciption 
#       The immediate value determines the number of bits to shift. The contents 
#	of RS1 is shifted left that many bits and the result is placed in RD. 
#	The bits shifted in are filled with zero.
#	For 32-bit machines, the shift amount must be within 0..31, 0 means no 
#	shifting is done.

	.text
	.global
_start:
	li x6, 1
	slli x5, x6
stop:
	j stop
	.end

