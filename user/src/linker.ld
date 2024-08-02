OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x10000;


SECTIONS
{
	. = BASE_ADDRESS;
	.text : {
		*(.text.entry)
		*(.text .text.*)
        }
	. = ALIGN(4k);
	.rodata : {
		*(.rodata .rodata.*)
		*(.srodata .srodata.*)	
        }
	. = ALIGN(4k);
	.data : {
		*(.data .data.*)
		*(.sdata .sdata.*)
 	}
	.bss : {
		start_bss = .;
		*(.bss .bss.*)
		*(.sbss .sbss.*)
		end_bss = .;

	}
	/DISCARD/ : {
		*(.eh_frame)
		*(.debug*)
	}


}
