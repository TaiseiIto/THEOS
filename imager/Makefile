TARGET = target/release/imager
TARGET_DIR = target
SOURCE = $(shell find src | grep ^.*\.rs$$)
LIBSTAT_ARCHIVE = lib/libstat.a
LIBSTAT_OBJECT = lib/libstat.o
LIBSTAT_SOURCE = lib/libstat.c

all: $(TARGET)

$(TARGET): $(SOURCE) $(LIBSTAT_ARCHIVE)
	cargo build --release

$(LIBSTAT_ARCHIVE): $(LIBSTAT_OBJECT) $(TARGET_DIR)
	ar -r $@ $<

$(LIBSTAT_OBJECT): $(LIBSTAT_SOURCE) $(TARGET_DIR)
	gcc $< -fPIC -c -o $@

$(TARGET_DIR):
	mkdir $@

