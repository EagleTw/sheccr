#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(while_true)]
// C language front-end 

use std::iter::Peekable;
use std::convert::TryInto;

// lexar tokens
#[derive(Debug,Clone)]
#[allow(dead_code)]
pub enum TokenT {
    Tstart, /* FIXME: it was intended to start the state machine. */
    Tnumeric(i64),                      /* Implimented */
    Tidentifier,                        
    Tcomma,  /* , */                    /* Implimented */
    Tstring(String), /* null-terminated string */
    Tchar(char),                        /* Implimented */                        
    TopenBracket,   /* ( */             /* Implimented */
    TcloseBracket,  /* ) */             /* Implimented */
    TopenCurly,     /* { */             /* Implimented */
    TcloseCurly,    /* } */             /* Implimented */
    TopenSquare,    /* [ */             /* Implimented */
    TcloseSquare,   /* ] */             /* Implimented */
    Tasterisk,      /* '*' */           /* Implimented */  
    TbitOr,         /* | */             /* Implimented */
    TlogAnd,        /* && */            /* Implimented */
    TlogOr,         /* || */            /* Implimented */
    TlogNot,        /* ! */             /* Implimented */
    Tlt,            /* < */             /* Implimented */
    Tgt,            /* > */             /* Implimented */
    Tle,            /* <= */            /* Implimented */
    Tge,            /* >= */            /* Implimented */
    Tlshift,        /* << */            /* Implimented */
    Trshift,        /* >> */            /* Implimented */
    Tdot,           /* . */             /* Implimented */
    Tarrow,         /* -> */            /* Implimented */
    Tplus,          /* + */             /* Implimented */
    Tminus,         /* - */             /* Implimented */
    Tminuseq,       /* -= */            /* Implimented */
    Tpluseq,        /* += */            /* Implimented */
    Toreq,          /* |= */            /* Implimented */
    Tandeq,         /* &= */            /* Implimented */
    Teq,            /* == */            /* Implimented */
    Tnoteq,         /* != */            /* Implimented */
    Tassign,        /* = */             /* Implimented */
    Tincrement,     /* ++ */            /* Implimented */
    Tdecrement,     /* -- */            /* Implimented */
    Tcolon,         /* : */             /* Implimented */
    Tsemicolon,     /* ; */             /* Implimented */
    Teof,           /* end-of-file (EOF) */             /* Implimented */
    Tampersand,     /* & */             //              /* Implimented */
    Treturn,
    Tif,
    Telse,
    Twhile,
    Tfor,
    Tdo,
    Tdefine,
    Tinclude,
    Ttypedef,
    Tenum,
    Tstruct,
    Tsizeof,
    Telipsis,       /* ... */
    Tswitch,
    Tcase,
    Tbreak,
    Tdefault,
}

pub fn lexer(input: &String) -> Result<Vec<TokenT>, String>{
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek(){
        match c{
            ' '|'\r'|'\n'|'n' => {it.next();}
            '0'..='9' => {
                it.next();
                let n = get_number(c,&mut it);
                result.push(TokenT::Tnumeric(n.try_into().unwrap()));
            }
            'A'..='Z' | 'a'..='z'=> {
                let st:String = get_till_space(c, &mut it);
                match st {
                    "return" => {result.push(TokenT::Treturn)}
                    // Tif,
                    // Telse,
                    // Twhile,
                    // Tfor,
                    // Tdo,
                    // Tdefine,
                    // Tinclude,
                    // Ttypedef,
                    // Tenum,
                    // Tstruct,
                    // Tsizeof,
                    // Telipsis,       /* ... */
                    // Tswitch,
                    // Tcase,
                    // Tbreak,
                    // Tdefault,
                    _ => {}
                }
            }
            // char 
            '\'' => {
                it.next();
                let ch:char = c;
                it.next();
                match c {
                    '\'' => {result.push(TokenT::Tchar(ch));}
                    _ => {return Err(format!("Is not char"));}
                }
            }
            ',' => {
                result.push(TokenT::Tcomma);
                it.next();
            }
            '(' => {
                result.push(TokenT::TopenBracket);
                it.next();
                }
            ')' => {
                result.push(TokenT::TcloseBracket);
                it.next();
            }
            '[' => {
                result.push(TokenT::TopenSquare);
                it.next();
            }
            ']' => {
                result.push(TokenT::TcloseSquare);
                it.next();
            }
            '{' => {
                result.push(TokenT::TopenCurly);
                it.next();
            }
            '}' => {
                result.push(TokenT::TcloseCurly);
                it.next();
            }
            
            '*' => {
                result.push(TokenT::Tampersand); 
                it.next();
            }
            '|' => {
                it.next();
                match c{
                    '|' => {
                        result.push(TokenT::TlogOr);
                        it.next();
                    }
                    '=' => {
                        result.push(TokenT::Toreq);
                        it.next();
                    }
                    _ => {result.push(TokenT::TbitOr);}
                }
            }
            '&' => {
                it.next();
                match c{
                    '&' => {
                        result.push(TokenT::TlogAnd);
                        it.next();
                    }
                    '=' => {
                        result.push(TokenT::Tandeq);
                        it.next();
                    }
                    _ => {
                        result.push(TokenT::Tampersand);
                    }
                }
            }
            '!' => {
                it.next();
                match c{
                    '=' => {
                        result.push(TokenT::Tnoteq);
                        it.next();
                    }
                    _ => {result.push(TokenT::TlogNot);}
                }
            }
            '<' => {
                it.next();
                match c{
                    '<' => {
                        result.push(TokenT::Tlshift);
                        it.next();
                    }
                    '=' => {
                        result.push(TokenT::Tle);
                        it.next();
                    }
                    _ => {result.push(TokenT::Tlt);}
                }
            }
            '>' => {
                it.next();
                match c{
                    '>' => {
                        result.push(TokenT::Trshift);
                        it.next();
                    }
                    '=' => {
                        result.push(TokenT::Tge);
                        it.next();
                    }
                    _ => {result.push(TokenT::Tgt);}
                }
            }
            '.' => {
                result.push(TokenT::Tdot);
                it.next();
            }
            '+' => {
                it.next();
                match c{
                    '+' => {
                        result.push(TokenT::Tincrement);
                        it.next();
                    }
                    '=' => {
                        result.push(TokenT::Tpluseq);
                        it.next();
                    }
                    _ => {result.push(TokenT::Tplus);}
                }
            }
            '-' => {
                it.next();
                match c{
                    '-' => {
                        result.push(TokenT::Tdecrement);
                        it.next();
                    }
                    '=' => {
                        result.push(TokenT::Tminuseq);
                        it.next();
                    }
                    '>' => {
                        result.push(TokenT::Tarrow);
                        it.next();
                    }
                    _ => {result.push(TokenT::Tminus);}
                }
            }
            '=' => {
                it.next();
                match c{
                    '=' => {
                        result.push(TokenT::Teq);
                        it.next();
                    }
                    _ => {result.push(TokenT::Tassign);}
                }
            }
            ':' => {
                result.push(TokenT::Tcolon); 
                it.next();
            }
            ';' => {
                result.push(TokenT::Tsemicolon); 
                it.next();
            }
            _ => {return Err(format!("unexpected character {}", c));}
        }
    }
    result.push(TokenT::Teof);
    Ok(result)
}

pub fn get_till_space<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> String {
    let mut output: String = c.to_string();
    iter.next();
    loop {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' => {
                output.push(c);
                iter.next();
            }
            _ => {break;}
        }
    }
    output
}

pub fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u64 {
    let mut number = c.to_string().parse::<u64>().expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u64>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

pub fn print_tokens(token_vec : &Vec<TokenT>) {
    println!("=====Resulting tokens=====");
    for token in &*token_vec {
    println!("Token: {:?}", token);
    }
    println!("=====End of tokens=====");
    println!();
}