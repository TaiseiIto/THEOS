legacy_bios_stack:
	.code16
	.text
legacy_bios_entry:
	jmp legacy_bios_main
	nop
legacy_bios_main:
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
	leave

