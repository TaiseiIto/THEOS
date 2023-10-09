# This makefile is called from ../Makefile
# Don't execute it directly.

QEMU = qemu-system-x86_64
ENABLE_5_LEVEL_PAGING = -cpu qemu64,+la57
LOG_FILE = ../qemu.log
LOG = -d int,cpu_reset -D $(LOG_FILE)
NO_REBOOT = --no-reboot
OVMF_CODE = ../qemu/roms/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_CODE.fd
OVMF_CODE_OPTION = -drive file=$(OVMF_CODE),format=raw,if=pflash
OVMF_VARS = ../qemu/roms/edk2/Build/OvmfX64/DEBUG_GCC5/FV/OVMF_VARS.fd
OVMF_VARS_OPTION = -drive file=$(OVMF_VARS),format=raw,if=pflash
THEOS = ../theos.img
THEOS_OPTION = -drive file=$(THEOS),format=raw,id=theos,if=none -device ide-hd,drive=theos,bootindex=1
SERIAL_LOG = ../serial.log
SERIAL_OPTION = -serial stdio
VNC_OPTION = -vnc :0
DEBUG_PORT = 2159
DEBUG_OPTION = -S -gdb tcp::$(DEBUG_PORT)

# Run THEOS on QEMU.
run:
	$(QEMU) $(ENABLE_5_LEVEL_PAGING) $(THEOS_OPTION) $(OVMF_CODE_OPTION) $(OVMF_VARS_OPTION) $(LOG) $(NO_REBOOT) $(SERIAL_OPTION) $(VNC_OPTION) | tee $(SERIAL_LOG)

# Stop THEOS on QEMU.
stop:
	for i in $$(ps ax | grep $(QEMU) | grep -v grep | awk '{print $$1}'); do kill $$i; done

# Debug THEOS on QEMU.
debug:
	$(QEMU) $(ENABLE_5_LEVEL_PAGING) $(THEOS_OPTION) $(OVMF_CODE_OPTION) $(OVMF_VARS_OPTION) $(LOG) $(NO_REBOOT) $(SERIAL_OPTION) $(VNC_OPTION) $(DEBUG_OPTION) | tee $(SERIAL_LOG)

