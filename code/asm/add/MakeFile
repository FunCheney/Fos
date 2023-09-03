MAKEFLAGS = -sR
CC	:= riscv64-unknown-elf-gcc
CCFLAGES	:= -c -march=rv32imac -mabi=ilp32 -g -Wall
LKFLAGES	:= -march=rv32imac -mabi=ilp32
QEMU	:= qemu-riscv32
QFLAGES := -g 1234
RM := rm 
RMFLAGES := -rf 

ALLC_FILE := $(wildcard *.c)
ALLS_FILE := $(wildcard *.S)
ALLC_OBJS = $(ALLC_FILE:%.c=%.o)
ALLS_OBJS = $(ALLS_FILE:%.S=%.o)
ALL_OBJS  = $(ALLS_OBJS) $(ALLC_OBJS)
MAINEXEC = main.elf

.PHYANY: all clean build run

all: clean build link

build:$(ALL_OBJS)
run:
	$(QEMU) $(QFLAGES) $(MAINEXEC)
link:$(MAINEXEC)
clean:
	$(RM) $(RMFLAGES) *.elf *.o *.i *.s

CCSTR		= 	'CC -[M] 正在构建... '$<
PRINTCSTR 	=	@echo $(CCSTR)
%.o : %.c
	$(CC) $(CCFLAGES) -o $@ $<
	$(PRINTCSTR)
%.o : %.S
	$(CC) $(CCFLAGES) -o $@ $<
	$(PRINTCSTR)	
%.elf :
	$(CC) $(LKFLAGES) -o $@ $(ALL_OBJS)
	@echo 'CC -[M] 正在构建... '$@