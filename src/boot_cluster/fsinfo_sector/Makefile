LINKER = ld
LINKER_MAP_OPTION = -Map
LINKER_OUTPUT_OPTION = -o
LINKER_SCRIPT_OPTION = -T

TARGETS = fsinfo_sector.bin

all: $(TARGETS)

fsinfo_sector.bin: fsinfo_sector.ld
	$(LINKER) $(LINKER_MAP_OPTION) $(@:.bin=.map) $(LINKER_OUTPUT_OPTION) $@ $(LINKER_SCRIPT_OPTION) $<

