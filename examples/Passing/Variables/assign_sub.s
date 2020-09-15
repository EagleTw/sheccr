    .code32
    .globl    main
    .type main, @function

main:
    pushl    %ebp # Set up stack frame
    movl     %esp, %ebp
    movl     $5,  %eax # Constant integer reference
    pushl    %eax
    movl     $3,  %eax # Constant integer reference
    pushl     %eax # Begin generating assignment operators
    popl      %ecx
    movl     -4(%ebp), %eax
    subl     %ecx, %eax # -=
    movl     %eax, -4(%ebp) # Assigning new value
    movl     $1,  %eax # Constant integer reference
    pushl     %eax # Begin generating assignment operators
    popl      %ecx
    movl     -4(%ebp), %eax
    subl     %ecx, %eax # -=
    movl     %eax, -4(%ebp) # Assigning new value
    movl     -4(%ebp), %eax # Variable reference for a
    movl     %ebp, %esp # Close function
    popl     %ebp
    ret
    movl     $0, %eax # Default return value
    movl     %ebp, %esp # Deallocate any local variables on stack
    popl     %ebp
    ret
