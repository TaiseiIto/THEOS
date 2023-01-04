OUTPUT_FORMAT("binary");

BASE = 0x7c00;

SECTIONS {
	. = BASE;
	.boot_sector : {
		boot_sector.o
	}
	. = BASE;
	. += 0x200;
	. -= 0x2;
	.boot_signature : {
		SHORT(0xaa55);
	}
	/DISCARD/ : {
		*(.eh_frame)
		*(.note.gnu.property)
	}
}

