# SHECCR

- a Self-Hosting and Educational C Compiler in Rust inspired by [shecc](https://github.com/jserv/shecc)
- Learning Rust technique from [rustcc](https://github.com/ClementTsang/rustcc/)
- LLVM IR learned from [here](https://mapping-high-level-constructs-to-llvm-ir.readthedocs.io/en/latest/README.html)

Features
--------
Currently, shecc supports the following features:
 
- Unary operators (logical negation, bitwise complements, negation)
- Binary operators (basic arithmetic, bitwise operations, comparisons)
- Local variables (assignment, declaration, variable calling, postfix and prefix incrementing)
- If-else branching
- Ternary operator
- While loops, do-while loops, for loops, break, continue
- Function calling and creation

As of now, only supports variables of type int.

On Going 
--------
- [x] Lexer  
- [x] Parser  
- [ ] llvm IR   

Dependencies
------------
Rust and Cargo installed


Usage 
-----
```script
$ cargo run file.c
```
