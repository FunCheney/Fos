
	.text
	.global _start

_start:
	lui x5, 0x12345 # int x5 = 0X12345 << 12
	addi x5, x5, 0x678  # int x5 = 0x12345000 + 0x678

stop:
	j stop
	
	.end
