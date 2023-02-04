# This makefile is called from ../Makefile
# Don't execute it directly.

OVMF_CODE=~/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_CODE.fd
OVMF_VARS=~/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_VARS.fd
QEMU=qemu-system-x86_64
THEOS=../theos.img
VNC_OPTION=-vnc :0

# Run THEOS on QEMU.
run:
	$(QEMU) -drive if=pflash,file=$(OVMF_CODE) -drive if=pflash,file=$(OVMF_VARS) -hda $(THEOS)

