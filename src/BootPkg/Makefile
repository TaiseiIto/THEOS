HERE = $(dir $(abspath $(firstword $(MAKEFILE_LIST))))
PKG = $(lastword $(subst /, , $(HERE)))
EDK2 = ../../qemu/roms/edk2
EDK2_PKG = $(EDK2)/$(PKG)

$(EDK2_PKG):
	ln -s $(HERE) $(EDK2_PKG)

