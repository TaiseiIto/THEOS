	.code16
stack_floor:
jump_boot:
	jmp	boot_code
	nop

oem_name:
	.ascii	"MSWIN4.1"

bytes_per_sector:
	.word	0x0200

sectors_per_cluster:
	.byte	0x08

reserved_sectors:
	.word	0x0020

fats:
	.byte	0x02

root_directory_entries:
	.word	0x0000

sectors16:
	.word	0x0000

media:
	.byte	0xf0

sectors_per_fat16:
	.word	0x0000

sectors_per_track:
	.word	0x0000

heads:
	.word	0x0000

hidden_sectors:
	.long	0x00000000

sectors32:
	.long	0x00000000

sectors_per_fat32:
	.long	0x00000000

fat_flags:
	.word	0x0000

file_system_version:
	.word	0x0000

root_directory_cluster:
	.long	0x00000000

file_system_information_sector:
	.word	0x0001

backup_boot_sector:
	.word	0x0006

reserved0:
	.fill	0xc,	0x1,	0x00

drive_number:
	.byte	0x80

reserved1:
	.byte	0x00

boot_signature:
	.byte	0x29

volume_identifier:
	.long	0x00000000

volume_label:
	.ascii	"THEOS      "

file_system_type:
	.ascii	"FAT16   "

boot_code:	# Print error message when booting on legacy BIOS.
	movw	$stack_floor,%bp
	movw	%bp,	%sp
	pushw	$error_message
	call	puts		# puts(error_message);
infinite_loop:
	hlt
	jmp	infinite_loop

puts:		# void puts(const char *s);
	pushw	%bp
	movw	%sp,	%bp
	pushw	%si
	pushw	%di
	movw	0x04(%bp),%si	# %si = s;
	subw	$0x0002,%sp
	movw	%sp,	%di	# const char *%di;
0:
	movb	(%si),	%dl	# %dl = *%si;
	testb	%dl,	%dl
	jz	1f		# if(%dl == '\0')goto 1f;
	movb	%dl,	(%di)
	call	putchar		# putchar(%dl);
	incw	%si		# %si++;
	jmp	0b		# goto 0b;
1:
	addw	$0x0002,%sp
	popw	%di
	popw	%si
	leave
	ret			# return;

putchar:	# void putchar(char c);
	pushw	%bp
	movw	%sp,	%bp
	pushw	%bx
	movb	0x04(%bp),%al
	movb	$0x0e,	%ah
	movb	$0x0f,	%bl
	int	$0x10		# Put char on screen.
	popw	%bx
	leave
	ret			# return;

error_message:
	.string "THEOS can work on not legacy BIOS but UEFI.\r\n"

