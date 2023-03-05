TARGET = target/x86_64-unknown-uefi/debug/bootx64.efi
SOURCE = $(shell find src | grep ^.*\.rs$$)

all: $(TARGET)

$(TARGET): $(SOURCE)
	cargo build --target x86_64-unknown-uefi

