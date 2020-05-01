default: apps

OVMF_CODE=OVMF/OVMF_CODE.fd
OVMF_VARS=OVMF/OVMF_VARS.fd
BUILD_DIR=target/x86_64-unknown-uefi/debug

QEMU=qemu-system-x86_64
QEMU_ARGS= \
	-m 128M \
    -drive if=pflash,format=raw,readonly,file=$(OVMF_CODE) \
    -drive if=pflash,format=raw,file=$(OVMF_VARS) \
    -drive format=raw,file=fat:rw:$(BUILD_DIR)\

apps: .FORCE
	cargo xbuild --target x86_64-unknown-uefi

.FORCE: 

run : apps
	$(QEMU) $(QEMU_ARGS)