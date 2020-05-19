default: BOOTX64.EFI

OVMF_CODE=../OVMF/OVMF_CODE.fd
OVMF_VARS=../OVMF/OVMF_VARS.fd
BUILD_DIR=target/x86_64-unknown-uefi/debug

QEMU=qemu-system-x86_64
QEMU_ARGS= \
	-m 10G \
    -drive if=pflash,format=raw,file=$(OVMF_CODE) \
    -drive if=pflash,format=raw,file=$(OVMF_VARS) \
    -drive format=raw,file=fat:rw:fs

BOOTX64.EFI: .FORCE
	mkdir -p fs/EFI/BOOT
	cargo xbuild --target x86_64-unknown-uefi
# 	rm fs/EFI/BOOT/BOOTX64.EFI
	cp $(BUILD_DIR)/uefi-bootloader.efi fs/EFI/BOOT/BOOTX64.EFI
# 	cp $(BUILD_DIR)/uefi-bootloader.efi fs/
	cp OVMF/OVMF_CODE.fd.orig $(OVMF_CODE)
	cp OVMF/OVMF_VARS.fd.orig $(OVMF_VARS)

.FORCE: 

run : BOOTX64.EFI
	$(QEMU) $(QEMU_ARGS)

qemu :
	$(QEMU) $(QEMU_ARGS)