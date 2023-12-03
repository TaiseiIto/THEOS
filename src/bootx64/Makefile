TARGET = target/x86_64-unknown-uefi/debug/bootx64.efi

all: $(TARGET)

$(TARGET): $(shell git ls-files)
	cargo build --target x86_64-unknown-uefi

