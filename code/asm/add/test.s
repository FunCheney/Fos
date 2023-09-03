    .text
    .global _start

_start:
        li x6, 1
        li x7, 2
        add x5, x6, x7
stop:
        j stop

        .end
