OUTPUT_ARCH(riscv)
ENTRY(_start)

BASS_ADDRESS = 0x80400000;

SECTIONS
{
	. = BASS_ADDRESS;
	.text : {
		*(.text.entry)
		*(.text .text.*)
        }

	.rodata : {
		*(.rodata .rodata.*)
		*(.srodata .srodata.*)	
        }
	.data : {
		*(.data .data.*)
		*(.sdata .sdata.*)
 	}
	.bss : {
		start_bss = .;
		*(.bss .bss.*)
		*(.sbss .sbss.*)
		ebss = .;
	}
	/DISCARD/ : {
		*(.en_farme)
		*(.debug*)
	}


}
