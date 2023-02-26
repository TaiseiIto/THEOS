HERE = $(dir $(abspath $(firstword $(MAKEFILE_LIST))))
HERE_MAKEFILE = edk2.mk
HERE_TARGET = target.txt
PKG = $(lastword $(subst /, , $(HERE)))
EDK2 = ../../qemu/roms/edk2
EDK2_PKG = $(EDK2)/$(PKG)
EDK2_MAKEFILE = $(EDK2)/Makefile
EDK2_TARGET = $(EDK2)/Conf/target.txt

all: $(EDK2_PKG)
	cp $(HERE_MAKEFILE) $(EDK2_MAKEFILE)
	cp $(HERE_TARGET) $(EDK2_TARGET)
	make -C $(EDK2)

$(EDK2_PKG):
	ln -s $(HERE) $(EDK2_PKG)

