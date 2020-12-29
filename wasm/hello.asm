	.text
	.file	"main"
	.section	.text.main,"",@
	.globl	main
	.type	main,@function
main:
	.functype	main () -> ()
	.local  	i32, i32, i32
	i32.const	__unnamed_1
	local.set	0
	i32.const	0
	local.set	1
	local.get	0
	local.get	1
	call	jsprint
	drop
	i32.const	0
	local.set	2
	local.get	2
	return
	end_function
.Lfunc_end0:
	.size	main, .Lfunc_end0-main

	.type	__unnamed_1,@object
	.section	.data.__unnamed_1,"",@
__unnamed_1:
	.ascii	"hello, world!"
	.size	__unnamed_1, 13

	.functype	jsprint (i32, i32) -> (i32)
