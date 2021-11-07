OUTPUT_FORMAT("binary");

INCLUDE ../../global.ld

BASE = LOADDEST + BOOT_SECTOR_SIZE;

SECTIONS
{
	. = BASE;
	.fsinfo_sign_1 :
	{
		LONG(0x41615252);
	}
	. += 0x00000000000001e0;
	.fsinfo_sign_2 :
	{
		LONG(0x61417272);
	}
	.free_clusters :
	{
		LONG(0xffffffff);
	}
	.last_cluster :
	{
		LONG(0xffffffff);
	}
	. += 0x000000000000000c;
	.fsinfo_sign_3 :
	{
		LONG(0xaa550000);
	}
}

