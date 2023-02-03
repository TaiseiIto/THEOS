TARGET=BOOTX64.EFI
SOURCE=$(TARGET:.EFI=.TXT)

all: $(TARGET)

$(TARGET): $(SOURCE)
	xxd -p -r $^ > $@

