    .code32
    .globl    main
    .type main, @function

main:
    pushl    %ebp # Set up stack frame
    movl     %esp, %ebp
    movl     $5,  %eax # Constant integer reference
    pushl    %eax
    movl     $1,  %eax # Constant integer reference
    pushl   %eax # Generating binary (+)
    movl     $2,  %eax # Constant integer reference
    popl    %ecx
    addl    %ecx, %eax # End +
    pushl   %eax # Generating binary (+)
    movl     $3,  %eax # Constant integer reference
    popl    %ecx
    addl    %ecx, %eax # End +
    pushl    %eax

loop1:
    movl     -8(%ebp), %eax # Variable reference for i
    pushl    %eax # Generating rel: <
    movl     $5,  %eax # Constant integer reference
    pushl    %eax # Generating binary (*)
    movl     $6,  %eax # Constant integer reference
    popl     %ecx
    imul     %ecx, %eax # End *
    popl     %ecx
    cmpl     %eax, %ecx
    movl     $0, %eax
    setl     %al # End <
    cmpl     $0, %eax
    je       post_loop3
    movl     -8(%ebp), %eax # Variable reference for i
    pushl     %eax # Begin generating assignment operators
    popl      %ecx
    movl     -4(%ebp), %eax
    addl     %ecx, %eax # +=
    movl     %eax, -4(%ebp) # Assigning new value
    addl     $0, %esp # Deallocate bytes

continue2:
    movl     -8(%ebp), %eax # Variable reference for i
    pushl   %eax # Generating binary (+)
    movl     $1,  %eax # Constant integer reference
    pushl    %eax # Generating binary (*)
    movl     $2,  %eax # Constant integer reference
    popl     %ecx
    imul     %ecx, %eax # End *
    pushl   %eax # Generating binary (+)
    movl     $3,  %eax # Constant integer reference
    popl    %ecx
    addl    %ecx, %eax # End +
    popl    %ecx
    addl    %ecx, %eax # End +
    movl     %eax, -8(%ebp) # Assigning new value
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
