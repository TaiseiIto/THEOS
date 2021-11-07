OUTPUT_FORMAT("binary");

INCLUDE global.ld

BASE = LOADDEST

SECTION
{
	. = BASE;
	.text:
	{
		boot_sector.o(.text)
		boot_sector.o(.data)
	}
	. = BASE;
	. += 0x00000000000001fe;
	.boot_sector_sign:
	{
		BYTE(0x55);
		BYTE(0xaa);
	}
	/DISCARD/:
	{
		*(.eh_frame)
	}
}

