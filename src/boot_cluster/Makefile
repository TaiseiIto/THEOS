TARGETS = boot_sector.bin

all: $(TARGETS)

boot_sector.bin: boot_sector/boot_sector.bin
	cp $^ $@

boot_sector/boot_sector.bin:
	make -C $(dir $@)

clean:
	make clean -C boot_sector
	../../script/clean.sh $(TARGETS)

rebuild: clean
	make

.PHONY: all clean rebuild

.PRECIOUS: %.bin

