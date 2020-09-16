    .code32
    .globl    main
    .type main, @function

main:
    pushl    %ebp # Set up stack frame
    movl     %esp, %ebp
    movl     $100,  %eax # Constant integer reference
    pushl    %eax
    movl     $5,  %eax # Constant integer reference
    pushl    %eax
    movl     -8(%ebp), %eax # Variable reference for a
    pushl   %eax # Generating binary (+)
    movl     -4(%ebp), %eax # Variable reference for i
    popl    %ecx
    addl    %ecx, %eax # End +
    pushl    %eax
    movl     -8(%ebp), %eax # Variable reference for a
    movl     %ebp, %esp # Close function
    popl     %ebp
    ret
    addl     $4, %esp # Deallocate bytes
    addl     $4, %esp # Deallocate bytes
    movl     $100,  %eax # Constant integer reference
    movl     %ebp, %esp # Close function
    popl     %ebp
    ret
    movl     $0, %eax # Default return value
    movl     %ebp, %esp # Deallocate any local variables on stack
    popl     %ebp
    ret
