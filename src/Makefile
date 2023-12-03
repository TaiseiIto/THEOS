BOOT_SECTOR = boot_sector
BOOTX64 = bootx64
KERNEL = kernel

all: $(shell git ls-files)
	make -C $(BOOT_SECTOR)
	make -C $(BOOTX64)
	make -C $(KERNEL)

