# First RISC-V

.macro do_nothing
        nop
        nop
.endm

        .text
        .global _start

_start:
        li x6, 5
        li x7, 4
        add x5, x6, x7
        do_nothing

stop: j stop

      .end
      