# This makefile is called from ../Makefile
# Don't execute it directly.

OVMF_CODE=/root/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_CODE.fd
OVMF_VARS=/root/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_VARS.fd
QEMU=qemu-system-x86_64
THEOS=/root/THEOS/theos.img
VNC_OPTION=-vnc :0

# Run THEOS on QEMU.
run:
	$(QEMU) -drive if=pflash,format=raw,file=$(OVMF_CODE) -drive if=pflash,format=raw,file=$(OVMF_VARS) -drive if=ide,format=raw,file=$(THEOS)

