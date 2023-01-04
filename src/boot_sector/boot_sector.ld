OUTPUT_FORMAT("binary");

BASE = 0x7c00;

SECTIONS {
	. = BASE;
	.boot_sector : {
		boot_sector.o
	}
	/DISCARD/ : {
		*(.eh_frame)
		*(.note.gnu.property)
	}
}

