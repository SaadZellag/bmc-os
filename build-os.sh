#!/usr/bin/sh
set -e
# cargo rustc --bin main --release -- --emit=obj
cargo rustc --crate-type=staticlib --release

nasm -f elf32 src/asm/bootloader.asm -f bin -o /tmp/boot_sect.bin
nasm -f elf32 src/asm/kernel_entry.asm -f elf -o /tmp/kernel_entry.bin
ld -m elf_i386 -o /tmp/kernel.bin -Ttext 0x1000 /tmp/kernel_entry.bin target/x86/release/libbmc_os.a --ignore-unresolved-symbol _GLOBAL_OFFSET_TABLE_ --oformat binary
cat /tmp/boot_sect.bin /tmp/kernel.bin > bmc-os.img

dd if=/dev/zero of=bmc-os.img bs=1 count=1 seek=524287
