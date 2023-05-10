
OBJS=objs
BINS=bins

all: run

$(BINS)/boot_sect.bin: src/asm/bootloader.asm src/asm/*.asm
	nasm -f elf32 $< -f bin -o $@

$(OBJS)/kernel_entry.o: src/asm/kernel_entry.asm 
	nasm -f elf32 $< -f elf -o $@

$(BINS)/kernel.bin: $(OBJS)/kernel_entry.o build_os
	ld -m elf_i386 -o $@ -Ttext 0x7F00 $< target/x86/release/libbmc_os.a --ignore-unresolved-symbol _GLOBAL_OFFSET_TABLE_ --oformat binary

build_os:
	cargo rustc --crate-type=staticlib --release

bmc-os.img: $(BINS)/boot_sect.bin $(BINS)/kernel.bin
	cat $^ > bmc-os.img
	dd if=/dev/zero of=bmc-os.img bs=1 count=1 seek=524287

run: bmc-os.img
	killall qemu-system-i386
	qemu-system-i386 -drive format=raw,file=bmc-os.img &
	sleep 0.1
	vncviewer :5900

clean:
	rm $(OBJS)/*
	rm $(BINS)/*