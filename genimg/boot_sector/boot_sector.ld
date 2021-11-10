OUTPUT_FORMAT("binary");

INCLUDE ../global.ld

BASE = LOADDEST;

SECTIONS
{
	. = BASE;
	.text :
	{
		boot_sector.o(.text)
		boot_sector.o(.data)
	}
	. = BASE;
	. += 0x00000000000001fe;
	.boot_sector_sign :
	{
		SHORT(0xaa55);
	}
	/DISCARD/:
	{
		*(.eh_frame)
	}
}

