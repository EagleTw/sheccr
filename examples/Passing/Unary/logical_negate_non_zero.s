    .code32
    .globl    main
    .type main, @function

main:
    pushl    %ebp # Set up stack frame
    movl     %esp, %ebp
    movl     $5,  %eax # Constant integer reference
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    movl     %ebp, %esp # Close function
    popl     %ebp
    ret
    movl     $0, %eax # Default return value
    movl     %ebp, %esp # Deallocate any local variables on stack
    popl     %ebp
    ret
