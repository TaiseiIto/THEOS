HERE = $(dir $(abspath $(firstword $(MAKEFILE_LIST))))
HERE_BUILD = build.sh
HERE_MAKEFILE = edk2.mk
HERE_TARGET = target.txt
PKG = $(lastword $(subst /, , $(HERE)))
EDK2 = ../../qemu/roms/edk2
EDK2_BUILD = $(EDK2)/build.sh
EDK2_OUTPUT = $(EDK2)/Build/BootX64/DEBUG_CLANG38/X64/Boot.efi
EDK2_PKG = $(EDK2)/$(PKG)
EDK2_MAKEFILE = $(EDK2)/Makefile
EDK2_TARGET = $(EDK2)/Conf/target.txt
OUTPUT = BOOTX64.EFI

all: $(OUTPUT)

$(OUTPUT): $(EDK2_OUTPUT)
	cp $^ $@

$(EDK2_OUTPUT): $(EDK2_BUILD) $(EDK2_MAKEFILE) $(EDK2_PKG) $(EDK2_TARGET)
	make -C $(EDK2)
	rm $^

$(EDK2_BUILD): $(HERE_BUILD)
	cp $^ $@

$(EDK2_MAKEFILE): $(HERE_MAKEFILE)
	cp $^ $@

$(EDK2_PKG):
	ln -s $(HERE) $(EDK2_PKG)

$(EDK2_TARGET): $(HERE_TARGET)
	cp $^ $@

