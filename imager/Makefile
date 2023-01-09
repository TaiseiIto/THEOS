TARGET = target/release/imager
SOURCE = $(shell find src | grep ^.*\.rs$$)
LIBSTAT_ARCHIVE = lib/libstat.a
LIBSTAT_OBJECT = lib/libstat.o
LIBSTAT_SOURCE = lib/libstat.c

all: $(TARGET)

$(TARGET): $(SOURCE) $(LIBSTAT_ARCHIVE)
	cargo build --release

$(LIBSTAT_ARCHIVE): $(LIBSTAT_OBJECT)
	ar -r $@ $^

$(LIBSTAT_OBJECT): $(LIBSTAT_SOURCE)
	gcc $^ -fPIC -c -o $@

