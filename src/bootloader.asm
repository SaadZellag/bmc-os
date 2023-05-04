
mov ah, 0x0e

mov al, 'H'
int 0x10
mov al, 'e'
int 0x10
mov al, 'l'
int 0x10
mov al, 'l'
int 0x10
mov al, 'o'
int 0x10
mov al, ' '
int 0x10
mov al, 'F'
int 0x10
mov al, 'r'
int 0x10
mov al, 'o'
int 0x10
mov al, 'm'
int 0x10
mov al, ' '
int 0x10
mov al, 'C'
int 0x10
mov al, 'A'
int 0x10
mov al, 'R'
int 0x10
mov al, 'L'
int 0x10

jmp $

times 510 -($-$$) db 0

dw 0xaa55