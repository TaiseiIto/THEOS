# This makefile is called from ../Makefile
# Don't execute it directly.

QEMU = qemu-system-x86_64
OVMF_CODE = ../qemu/roms/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_CODE.fd
OVMF_CODE_OPTION = -drive file=$(OVMF_CODE),format=raw,if=pflash
OVMF_VARS = ../qemu/roms/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_VARS.fd
OVMF_VARS_OPTION = -drive file=$(OVMF_VARS),format=raw,if=pflash
THEOS = ../theos.img
THEOS_OPTION = -drive file=$(THEOS),format=raw,id=theos,if=none -device ide-hd,drive=theos,bootindex=1
SERIAL_OPTION = -serial stdio
VNC_OPTION = -vnc :0

# Run THEOS on QEMU.
run:
	$(QEMU) $(THEOS_OPTION) $(OVMF_CODE_OPTION) $(OVMF_VARS_OPTION) $(SERIAL_OPTION) $(VNC_OPTION)

# Stop THEOS on QEMU.
stop:
	for i in $$(ps ax | grep $(QEMU) | grep -v grep | awk '{print $$1}'); do kill $$i; done

