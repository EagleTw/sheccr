pub mod parser;
//pub mod x86_codegen;
use std::env;

fn main() {
    // Convert our .c file into an AST.
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let parsed_prog : parser::Program = parser::parse_to_ast(filename);

    // Convert AST into valid assembly.
    //x86_codegen::generate_assembly(&parsed_prog, filename.clone());

}
