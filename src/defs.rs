// definitions 

// limitations
static MAX_TOKEN_LEN:i8 = 256;
static MAX_ID_LEN:i8 = 64;
static MAX_LINE_LEN:i8 = 256;
static MAX_VAR_LEN:i8 = 64;
static MAX_TYPE_LEN:i8 = 64;
static MAC_PARAMS:i8 = 8;
static MAX_LOCALS:i8 = 48;
static MAX_FIELD:i8 = 32;
static MAX_FUNCT:i16 = 1024;
static MAX_BLOCKS:i16 = 65535;
static MAX_TYPES:i8 = 64;
static MAX_IR_INSTR:i16 = 65535;
static MAX_SOURCE:i32 = 131072;
static MAX_CODE:i32 = 131072;
static MAX_DATA:i32 = 131072;


enum BaseTypeT {
    TypeVoid = 0,
    TypeInt,
    TypeChar,
    TypeCtruct,
}

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

