TARGET = target/x86_64-unknown-uefi/debug/loeader.efi
SOURCE = $(shell find src | grep ^.*\.rs$$)

all: $(TARGET)

$(TARGET): $(SOURCE)
	cargo build --target x86_64-unknown-uefi

