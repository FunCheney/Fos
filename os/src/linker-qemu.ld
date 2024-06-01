OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
	. = BASE_ADDRESS;
	skernel = .;

	stext = .;
	.text : {
	        *(.text.entry)
      		*(.text .text.*) 	      
	}
	
	. = ALIGN(4k);
	etext = .;
	srodata = .;
	.rodata : {
		*(.rodata .rodata.*)
		*(.srodata .srodata.*)
	}
	
	. = ALIGN(4k);
	erodata = .;
	sdata = .;
	.data : {
		*(.data .data.*)
		*(.sdata .sdata.*)
	}

	. = ALIGN(4k);
	edata = .;
	.bss : {
	       *(.bss.stack)
	       sbss = .;
	       *(.bss .bss.*)
	       *(.sbss .sbss.*)
	}

	. = ALIGN(4k);
	ebss = .;
	ekernel = .;

	/DISCARD/ : {
		*(.eh_frame)
        }
}
