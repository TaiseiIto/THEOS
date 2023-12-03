TARGET = target/debug/kernel

all: $(TARGET)

$(TARGET): $(shell git ls-files)
	rustup component add rust-src --toolchain nightly-2023-02-25-x86_64-unknown-linux-gnu
	cargo build

