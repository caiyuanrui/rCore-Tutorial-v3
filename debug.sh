#/bin/bash

set -e

qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ./rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
    -s -S
