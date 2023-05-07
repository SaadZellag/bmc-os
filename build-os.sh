#!/usr/bin/sh
set -e
cd os
cargo build --target-dir ../target --release
cd ..

# nasm bootloader.asm -f bin -o ../../bmc-os.img

# nasm mbr.asm -f bin -o ../../bmc-os.img

# ld -m elf_i386 -o /tmp/kernel.bin -Ttext 0x1000 target/x86/debug/os --oformat binary

cd src/asm
nasm -f elf32 bootloader.asm -f bin -o /tmp/boot_sect.bin
nasm -f elf32 kernel_entry.asm -f elf -o /tmp/kernel_entry.bin
cd ../..
ld -m elf_i386 -o /tmp/kernel.bin -Ttext 0x1000 /tmp/kernel_entry.bin target/x86/release/os --oformat binary
cat /tmp/boot_sect.bin /tmp/kernel.bin > bmc-os.img


# nasm mbr.asm -f bin -o /tmp/mbr.bin
# nasm bootloader.asm -f elf -o /tmp/bootloader.bin
# cd ../..
# ld -m elf_i386 -o /tmp/kernel.bin -Ttext 0x1000 /tmp/bootloader.bin target/x86/debug/os --oformat binary
# cat /tmp/mbr.bin /tmp/kernel.bin > bmc-os.img

dd if=/dev/zero of=bmc-os.img bs=1 count=1 seek=524287

# cargo run --release --bin assembleos target/x86_64/debug/os