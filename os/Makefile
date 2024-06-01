# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := traget/${TARGET}/$(MODE)/os
KERNEL_BIN := $(KERNEL_ELF).bin 
DISAM_TMP := traget/$(TARGET)/$(MODE)/asm

# Building mode argumet
ifeq ($(MODE), release)
	MODE_ARG := --release

endif

# BOARD
BOARD := qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

# KERNEL ENTRY
KERNEL_ENTRY_PA := 0x80200000

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Disassembly
DISAM ?= -x

build: env $(KERNEL_BIN)

env: 









