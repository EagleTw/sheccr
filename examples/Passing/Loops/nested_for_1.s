    .code32
    .globl    main
    .type main, @function

main:
    pushl    %ebp # Set up stack frame
    movl     %esp, %ebp
    movl     $0,  %eax # Constant integer reference
    pushl    %eax
    movl     $0,  %eax # Constant integer reference
    pushl    %eax

loop1:
    movl     -8(%ebp), %eax # Variable reference for i
    pushl    %eax # Generating rel: <
    movl     $5,  %eax # Constant integer reference
    popl     %ecx
    cmpl     %eax, %ecx
    movl     $0, %eax
    setl     %al # End <
    cmpl     $0, %eax
    je       post_loop3
    movl     $0,  %eax # Constant integer reference
    pushl    %eax

loop4:
    movl     -12(%ebp), %eax # Variable reference for j
    pushl    %eax # Generating rel: <
    movl     $10,  %eax # Constant integer reference
    popl     %ecx
    cmpl     %eax, %ecx
    movl     $0, %eax
    setl     %al # End <
    cmpl     $0, %eax
    je       post_loop6
    movl     -12(%ebp), %eax # Variable reference for j
    pushl     %eax # Begin generating assignment operators
    popl      %ecx
    movl     -4(%ebp), %eax
    addl     %ecx, %eax # +=
    movl     %eax, -4(%ebp) # Assigning new value
    addl     $0, %esp # Deallocate bytes

continue5:
    movl     -12(%ebp), %eax # Variable reference for j
    movl     -12(%ebp), %eax # Variable reference for ++(post)
    movl     %eax, %ecx
    addl     $1, %ecx
    movl     %ecx, -12(%ebp) # Variable assignment for ++(post)
    jmp      loop4

post_loop6:
    addl     $4, %esp # Deallocate bytes
    movl     -8(%ebp), %eax # Variable reference for i
    pushl     %eax # Begin generating assignment operators
    popl      %ecx
    movl     -4(%ebp), %eax
    subl     %ecx, %eax # -=
    movl     %eax, -4(%ebp) # Assigning new value
    addl     $0, %esp # Deallocate bytes

continue2:
    movl     -8(%ebp), %eax # Variable reference for i
    movl     -8(%ebp), %eax # Variable reference for ++(pre)
    addl     $1, %eax
    movl     %eax, -8(%ebp) # Variable assignment for ++(pre)
    jmp      loop1

post_loop3:
    addl     $4, %esp # Deallocate bytes
    movl     -4(%ebp), %eax # Variable reference for a
    movl     %ebp, %esp # Close function
    popl     %ebp
    ret
    movl     $0, %eax # Default return value
    movl     %ebp, %esp # Deallocate any local variables on stack
    popl     %ebp
    ret
