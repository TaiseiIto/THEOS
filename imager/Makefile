TARGET = target/release/imager
SOURCE = $(shell find src | grep ^.*\.rs$$)

all: $(TARGET)

$(TARGET): $(SOURCE)
	make -C ffi
	cargo build --release

