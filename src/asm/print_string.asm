print_string:
	mov ah, 0x0e
	start:
		mov al, [bx]
		int 0x10
		inc bx
		cmp al, 0x00
		jne start
	ret