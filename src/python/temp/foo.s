	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 14, 0
	.globl	_main                           ; -- Begin function main
	.p2align	2
_main:                                  ; @main
	.cfi_startproc
; %bb.0:                                ; %entry
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	mov	w8, #1
	mov	w9, #2
	mov	w10, #3
	mov	w0, wzr
	stp	w9, w8, [sp, #8]
	str	w10, [sp, #4]
	add	sp, sp, #16
	ret
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols
