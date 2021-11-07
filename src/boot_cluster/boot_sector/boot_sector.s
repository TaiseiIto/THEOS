# Bibliographies
# http://elm-chan.org/docs/fat.html
# https://wiki.osdev.org/FAT

# 0x0200 bytes per sector
# 0x0008 sectors per cluster
# 0x0001 clusters per track
# 0x0100 clusters per disk

# The disk layout
# cluster 0x01 : First boot cluster
#  sector 0x00 : Boot sector
#  sector 0x01 : FSInfo sector
# cluster 0x02 : Second boot cluster
#  sector 0x00 : Boot sector
#  sector 0x01 : FSInfo sector
# cluster 0x03 : First FAT
# cluster 0x04 : Second FAT
# cluster 0x05 : Root directory

	.set	bytes_per_sector,	0x0200
	.set	sectors_per_cluster,	0x08
	.set	boot_sectors,		0x0010
	.set	fats,			0x02
	.set	root_directory_entries,	0x0100
	.set	sectors,		0x0800	# The drive size = 1 MiB = 0x800 sectors * 0x0200 bytes
	.set	media_type,		0xf8
	.set	sectors_per_fat,	0x0008
	.set	sectors_per_track,	0x0008
	.set	heads,			0x0002
	.set	hidden_sectors,		0x00000000
	.set	flags,			0x0000
	.set	fat_version,		0x0000
	.set	root_directory_cluster,	0x00000005
	.set	fsinfo_sector,		0x0001
	.set	backup_boot_sector,	0x0008
	.set	drive_number,		0x80
	.set	boot_signature,		0x29
	.include	"volume_serial_number.s"

legacy_bios_stack:
	.code16
	.text
legacy_bios_entry:
	jmp legacy_bios_main
	nop
	.ascii	"THEOS   "
	.word	bytes_per_sector
	.byte	sectors_per_cluster
	.word	boot_sectors
	.byte	fats
	.word	root_directory_entries
	.word	sectors
	.byte	media_type
	.word	sectors_per_fat
	.word	sectors_per_track
	.word	heads
	.long	hidden_sectors
	.long	sectors
	.long	sectors_per_fat
	.word	flags
	.word	fat_version
	.long	root_directory_cluster
	.word	fsinfo_sector
	.word	backup_boot_sector
	.fill	0x0c, 0x01, 0x00	# Locate 0x0c bytes of 0x00
	.byte	drive_number
	.byte	0x00
	.byte	boot_signature
	.long	volume_serial_number
	.ascii	"THEOS      "
	.ascii	"FAT32   "
legacy_bios_main:
0:
	xorw	%ax,	%ax
	movw	%ax,	%bx
	movw	%ax,	%cx
	movw	%ax,	%dx
	movw	%ax,	%si
	movw	%ax,	%di
	movw	%ax,	%ss
	movw	%ax,	%ds
	movw	%ax,	%es
	movw	%ax,	%fs
	movw	%ax,	%gs
	movw	$legacy_bios_stack,	%bp
	movw	$legacy_bios_stack,	%sp
	pushw	%bp
	movw	%sp,	%bp
	pushw	%si
	pushw	%di
	subw	$0x0002,%sp
	movw	%sp,	%di
	movw	$message,(%di)
	call	print
1:
	hlt
	jmp	1b

print:		# void print(char *string);
0:
	pushw	%bp
	movw	%sp,	%bp
	pushw	%si
	pushw	%di
	subw	$0x0002,%sp
	movw	%sp,	%di
	movw	0x04(%bp),%si
1:
	xorb	%ah,	%ah
	movb	(%si),	%al
	cmpb	$0x00,	%al
	je	2f
	movw	%ax,	(%di)
	call	put_char
	incw	%si
	jmp	1b
2:
	addw	$0x0002,%sp
	popw	%di
	popw	%si
	leave
	ret

put_char:	# void put_char(char c);
0:
	pushw	%bp
	movw	%sp,	%bp
	pushw	%bx
	movb	0x04(%bp),%al
	movb	$0x0e,	%ah
	movw	$0x000f,%bx
	int	$0x10
	popw	%bx
	leave
	ret

	.data
message:
	.string	"THEOS is bootable on not legacy BIOS mode but only UEFI mode.\nPlease push the power button to power off the Computer and boot up on UEFI mode."

