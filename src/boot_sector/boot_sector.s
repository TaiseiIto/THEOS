        .boot_sector
stack_floor:
JumpBoot:
	.code16
	jmp	boot_code
	nop

boot_code:	# Print error message when booting on legacy BIOS.
	movw	$stack_floor,%bp
	movw	%bp,	%sp
	pushw	$error_message
	call	puts		# puts(error_message);
infinite_loop:
	hlt
	jmp	$infinite_loop

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

