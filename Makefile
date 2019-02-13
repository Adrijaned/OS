OBJECTS = loader.o io.o target/x86-unknown-adrij_os/debug/libadrij_os_rust.a segment.o interrupts.o
LDFLAGS = -T link.ld -melf_i386
AS = nasm
ASFLAGS = -f elf

all: kernel.elf

kernel.elf: $(OBJECTS)
	ld $(LDFLAGS) $(OBJECTS) -o kernel.elf

os.iso: kernel.elf
	cp kernel.elf iso/boot/kernel.elf
	genisoimage -R -b boot/grub/stage2_eltorito -no-emul-boot -boot-load-size 4 -A LittleBookOS -input-charset utf8 -boot-info-table -o os.iso iso

run: os.iso
	bochs -f bochsrc -q

%.o: %.s
	$(AS) $(ASFLAGS) $< -o $@

target/x86-unknown-adrij_os/debug/libadrij_os_rust.a:
	RUST_TARGET_PATH=$(shell pwd) xargo build --target=x86-unknown-adrij_os

clean: 
	rm -rf *.o kernel.elf os.iso iso/boot/kernel.elf target/x86-unknown-adrij_os/debug/libadrij_os_rust.a
