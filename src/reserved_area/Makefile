TARGETS = boot_sector.bin fsinfo_sector.bin

all: $(TARGETS)

boot_sector.bin: boot_sector/boot_sector.bin
	cp $^ $@

boot_sector/boot_sector.bin:
	make -C $(dir $@)

fsinfo_sector.bin: fsinfo_sector/fsinfo_sector.bin
	cp $^ $@

fsinfo_sector/fsinfo_sector.bin:
	make -C $(dir $@)

clean:
	make clean -C boot_sector
	make clean -C fsinfo_sector
	../../script/clean.sh $(TARGETS)

rebuild: clean
	make

.PHONY: all clean rebuild

.PRECIOUS: %.bin

