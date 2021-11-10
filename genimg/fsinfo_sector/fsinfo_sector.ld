OUTPUT_FORMAT("binary");

INCLUDE ../global.ld

BASE = LOADDEST + BOOT_SECTOR_SIZE;

SECTIONS
{
	. = BASE;
	.fsinfo :
	{
		fsinfo_sector.o(.data)
	}
}

