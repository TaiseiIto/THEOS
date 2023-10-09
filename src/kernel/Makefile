TARGET = target/debug/kernel
SOURCE = $(shell find src | grep ^.*\.rs$$)

all: $(TARGET)

$(TARGET): $(SOURCE)
	rustup component add rust-src --toolchain nightly-2023-02-25-x86_64-unknown-linux-gnu
	cargo build

