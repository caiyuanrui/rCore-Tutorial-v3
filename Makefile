TARGET = target/riscv64gc-unknown-none-elf/release/os
BIN = target/riscv64gc-unknown-none-elf/release/os.bin
QEMU_BIOS = ./rustsbi-qemu.bin

all: run

build:
	cargo build --release

strip: build
	rust-objcopy --strip-all $(TARGET) -O binary $(BIN)

run: strip
	qemu-system-riscv64 \
	  -machine virt \
	  -nographic \
	  -bios ./rustsbi-qemu.bin \
	  -device loader,file=$(BIN),addr=0x80200000

clean:
	cargo clean
