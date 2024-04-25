# load upper Immediate
# Foramt:
#    LUI RD, IMM 
# Description
#    The instruction contains a 20-bit immdeiate value. This value is placed 
#    in the leftmost (i.e., upper, most significant) 20 bits of the register
#    RD and the rightmost (i.e., lower, least significant) 12-bits are set

	.text            # Define beginning of text selection
	.global _start   # Define entry _start

_start:
	lui x5, 0x12345 # int x5 = 0X12345 << 12
	addi x5, x5, 0x678  # int x5 = 0x12345000 + 0x678

stop:
	j stop     # Infinite loop to stop execution
	
	.end       # End of file
