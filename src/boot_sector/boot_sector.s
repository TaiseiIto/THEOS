	.code16
stack_floor:
jump_boot:
	jmp	boot_code
	nop

file_system_name:
	.ascii	"EXFAT   "

must_be_zero:
	.fill	0x35,	0x1,	0x00

partition_offset:
	.quad	0x0000000000000000

volume_length:
	.quad	0x0000000000000000

fat_offset:
	.long	0x00000000

fat_length:
	.long	0x00000000

cluster_heap_offset:
	.long	0x00000000

cluster_count:
	.long	0x00000000

first_cluster_of_root_directory:
	.long	0x00000000

volume_serial_number:
	.long	0x00000000

file_system_revision:
	.word	0x0100

volume_flags:
	.word	0x0000

bytes_per_sector_shift:
	.byte	0x09

sectors_per_cluster_shift:
	.byte	0x08

number_of_fats:
	.byte	0x02

drive_select:
	.byte	0x80

percent_in_use:
	.byte	0xff

reserved:
	.fill	0x07,	0x1,	0x00

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
	.string "THEOS can work on not legacy BIOS but UEFI.\n"

