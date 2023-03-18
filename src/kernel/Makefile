TARGET = target/debug/kernel
SOURCE = $(shell find src | grep ^.*\.rs$$)

all: $(TARGET)

$(TARGET): $(SOURCE)
	cargo build --target x86_64-unknown-none

