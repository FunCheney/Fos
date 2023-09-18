	.text
	.global _start
_start:
	auipc x5, 0x12345 # x5 = 0x12345 << 12 + pc 
	auipc x6, 0       # x6 = pc, to obtain the pc
stop:
	j   stop
	.end
