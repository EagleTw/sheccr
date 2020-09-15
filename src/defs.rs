// definitions 

// limitations
static MAX_TOKEN_LEN:u16 = 256;
static MAX_ID_LEN:u8 = 64;
static MAX_LINE_LEN:u16 = 256;
static MAX_VAR_LEN:u8 = 64;
static MAX_TYPE_LEN:u16 = 64;
static MAC_PARAMS:u8 = 8;
static MAX_LOCALS:u8 = 48;
static MAX_FIELD:u8 = 32;
static MAX_FUNCT:u32 = 1024;
static MAX_BLOCKS:u32 = 65536;
static MAX_TYPES:u8 = 64;
static MAX_IR_INSTR:u32 = 65536;
static MAX_SOURCE:u32 = 131072;
static MAX_CODE:u32 = 131072;
static MAX_DATA:u32 = 131072;
static MAX_SYMTAB:u32 = 65536;
static MAX_STRTAB:u32 = 65536;
static MAX_HEADER:u16 = 1024;
static MAX_SECTION:u16 = 1024;
static MAX_ALIASES:u16 = 1024;
static MAX_CONSTANTS:u16 = 1024;
static MAX_CASES:u8 = 128;
static MAX_NESTING:u8 = 128;

static ELF_START:usize = 0x10000;
static PTR_SIZE: u8 = 4;

// builtin types
enum BaseTypeT {
    TypeVoid = 0,
    TypeInt,
    TypeChar,
    TypeCtruct,
}

// IR opcode
enum OpcodeT {
    // generic: intermediate use in front-end. No code generation
    OpGeneric,

    // calling convention
    OpFuncExtry,    // function entry point
    OpExit,         // program exit rutine
    OpCall,         // function call
    OpFuncExit,     // function exit code
    OpRuturn,       // jump to function exit

    OpLoadConstant,      //  load constant
    OpLoadDataAddress,   //  lookup address of constant in data section

    // stack operation
    OpPush,         // push into stack
    OpPop,          // pop from stack

    // control flow
    OpJump,         // unconditional jump
    OpLabel,        // note label
    OpJz,           // jump if false
    OpJnz,          // jump if true
    OpBlockStart,   // code block start
    OpBlockEnd,     // code block end

    // memory address operations
    OpAddressOf,    // lookup variable's address
    OpRead,         // read from memory address 
    OpWrite,        // write to memory 

    // arithmetic operators
    OpAdd,
    OpSub,
    OpMul,
    OpLSHift,
    OpRShift,
    OpLogAnd,
    OpLogOr,
    OpNot,
    OpEq,   // equal
    OpNeq,  // not equal
    OpLt,   // less than
    OpGt,   // greater than
    OpGeq,  // greater than or equal
    OpBitOr,
    OpBitAnd,
    OpNegate,

    // platform specific
    OpSysCall,
    OpStart,
}

// IR instruction
pub struct IRInstrT {
    op:OpcodeT,
    op_len: usize,
    ir_index: usize,
    code_offset: usize,
    param_no: usize,
    int_param1: usize,
    int_param2: usize,
    str_param1: str,         // not sure!
}

// variable definition 
pub struct VarT {
    type_name: String,
    var_name: String,
    is_ptr: bool,
    array_size: usize,
    offset:usize,     // offset from stack or frame
}

// function definition
pub struct FuncT {
    return_def: VarT,
    param_defs: VarT,
    num_params: usize,
    entry_point: usize,  // IR index
    exit_point: usize,   // IR index
    params_size: usize,   
}

// block definition
pub struct BlockT<'a> {
   locals: VarT,
   next_local: usize,
   parent: &'a mut BlockT<'a>,
   func_t: &'a mut FuncT,
   local_size: usize,
   index: usize,
}

// type definition
pub struct TypeT {
    type_name: String<>,
    base_type: BaseTypeT,
    size: usize,
    fields: VarT,
    num_fields: usize,
}

// lvalue details
pub struct LvalueT {
    size: usize,
    is_ptr: bool,
    r#type: TypeT,      // escape reserved keywords to use them as identifiers
}

// alias for #defines
pub struct AliasT {
    alias: String<>,
    value: String<>,
}

// constants for enum
pub struct ConstantT {
    alias: String<>,
    value: usize,
}