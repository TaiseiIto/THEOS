TARGET = target/release/imager

all: $(TARGET)

$(TARGET): $(shell git ls-files)
	cargo build --release

