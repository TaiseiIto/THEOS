# This makefile is called from ../Makefile
# Don't execute it directly.

OVMF_CODE=/root/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_CODE.fd
OVMF_VARS=/root/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_VARS.fd
QEMU=qemu-system-x86_64
THEOS=/root/THEOS/theos.img
VNC_OPTION=-vnc :0

# Run THEOS on QEMU.
run:
	$(QEMU) -drive file=$(THEOS),format=raw,id=theos,if=none -device ide-hd,drive=theos,bootindex=1 -drive file=$(OVMF_CODE),format=raw,if=pflash -drive file=$(OVMF_VARS),format=raw,if=pflash $(VNC_OPTION)

# Stop THEOS on QEMU.
stop:
	for i in $$(ps ax | grep $(EMULATOR) | grep -v grep | awk '{print $$1}'); do kill $$i; done

