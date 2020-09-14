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
    Tnumeric(i64),                      /* Implimented */       /* Tested */ 
    Tidentifier(String),                /* Implimented */       /* Tested */
    Tcomma,  /* , */                    /* Implimented */       /* Tested */
    Tstring(String), /* null-terminated string */
    Tchar(char),                        /* Implimented */       /* Tested */                               
    TopenBracket,   /* ( */             /* Implimented */       /* Tested */
    TcloseBracket,  /* ) */             /* Implimented */       /* Tested */
    TopenCurly,     /* { */             /* Implimented */       /* Tested */
    TcloseCurly,    /* } */             /* Implimented */       /* Tested */
    TopenSquare,    /* [ */             /* Implimented */       /* Tested */
    TcloseSquare,   /* ] */             /* Implimented */       /* Tested */
    Tasterisk,      /* '*' */           /* Implimented */       /* Tested */  
    TbitOr,         /* | */             /* Implimented */       /* Tested */
    TlogAnd,        /* && */            /* Implimented */       /* Tested */
    TlogOr,         /* || */            /* Implimented */       /* Tested */
    TlogNot,        /* ! */             /* Implimented */       /* Tested */
    Tlt,            /* < */             /* Implimented */       /* Tested */       
    Tgt,            /* > */             /* Implimented */       /* Tested */
    Tle,            /* <= */            /* Implimented */       /* Tested */
    Tge,            /* >= */            /* Implimented */       /* Tested */
    Tlshift,        /* << */            /* Implimented */       /* Tested */
    Trshift,        /* >> */            /* Implimented */       /* Tested */
    Tdot,           /* . */             /* Implimented */       /* Tested */
    Tarrow,         /* -> */            /* Implimented */       /* Tested */
    Tplus,          /* + */             /* Implimented */       /* Tested */
    Tminus,         /* - */             /* Implimented */       /* Tested */
    Tminuseq,       /* -= */            /* Implimented */       /* Tested */
    Tpluseq,        /* += */            /* Implimented */       /* Tested */
    Toreq,          /* |= */            /* Implimented */       /* Tested */
    Tandeq,         /* &= */            /* Implimented */       /* Tested */
    Teq,            /* == */            /* Implimented */       /* Tested */
    Tnoteq,         /* != */            /* Implimented */       /* Tested */
    Tassign,        /* = */             /* Implimented */       /* Tested */
    Tincrement,     /* ++ */            /* Implimented */       /* Tested */
    Tdecrement,     /* -- */            /* Implimented */       /* Tested */
    Tcolon,         /* : */             /* Implimented */       /* Tested */
    Tsemicolon,     /* ; */             /* Implimented */       /* Tested */
    Teof,           /* end-of-file*/    /* Implimented */       /* Tested */
    Tampersand,     /* & */             /* Implimented */       /* Tested */
    Treturn,                            /* Implimented */       /* Tested */
    Tif,                                /* Implimented */       /* Tested */
    Telse,                              /* Implimented */       /* Tested */
    Twhile,                             /* Implimented */       /* Tested */
    Tfor,                               /* Implimented */       /* Tested */
    Tdo,                                /* Implimented */       /* Tested */
    Tdefine,                            /* Implimented */       /* Tested */
    Tinclude,                           /* Implimented */       /* Tested */
    Ttypedef,                           /* Implimented */       /* Tested */
    Tenum,                              /* Implimented */       /* Tested */
    Tstruct,                            /* Implimented */       /* Tested */
    Tsizeof,                            /* Implimented */       /* Tested */
    Telipsis,      /* ... */            /* Implimented */       /* Tested */            
    Tswitch,                            /* Implimented */       /* Tested */
    Tcase,                              /* Implimented */       /* Tested */ 
    Tbreak,                             /* Implimented */       /* Tested */
    Tdefault,                           /* Implimented */       /* Tested */
}

pub fn get_till_space<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> String {
    let mut output: String = c.to_string();
    iter.next();
    while let Some(&st)= iter.peek(){
        match st{
            'A'..='Z' | 'a'..='z' | '0'..='9' => {
                output.push(st);
                iter.next();
            }
            _ => {break;}
        }
    }
    output
}

pub fn get_hex_number <T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u64{
    let mut output: u64;
    iter.next();
    while let Some(&s) = iter.peek(){
        match s {
            '0' => {output = output*16 + 0;}
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

pub fn lexer(input: &String) -> Result<Vec<TokenT>, String>{
    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek(){
        match c{
            ' '|'\r'|'\n'|'n' => {it.next();}
            '0'..='9' => {
                it.next();
                if (it.peek().unwrap() == &'x'){
                    get_hex_number(c, &mut it);
                }
                let n = get_number(c,&mut it);
                result.push(TokenT::Tnumeric(n.try_into().unwrap()));
            }
            'A'..='Z' | 'a'..='z'=> {
                let st:String = get_till_space(c, &mut it);
                //println!("{}",st);
                match st.as_ref(){
                    "return" => {result.push(TokenT::Treturn)}
                    "if"     => {result.push(TokenT::Tif)}
                    "else"   => {result.push(TokenT::Telse)}
                    "while"  => {result.push(TokenT::Twhile)}
                    "for"    => {result.push(TokenT::Tfor)}
                    "do"     => {result.push(TokenT::Tdo)} 
                    "define" => {result.push(TokenT::Tdefine)}
                    "include"=> {result.push(TokenT::Tinclude)}
                    "typedef"=> {result.push(TokenT::Ttypedef)}
                    "enum"   => {result.push(TokenT::Tenum)}
                    "struct" => {result.push(TokenT::Tstruct)}
                    "sizeof" => {result.push(TokenT::Tsizeof)}
                    "switch" => {result.push(TokenT::Tsizeof)}
                    "case"   => {result.push(TokenT::Tcase)}
                    "break"  => {result.push(TokenT::Tbreak)}
                    "default"=> {result.push(TokenT::Tdefine)}
                    _ => {result.push(TokenT::Tidentifier(st));}
                }
            }
            // char 
            '\'' => {
                it.next();
                let &ch = it.peek().unwrap();
                println!("{},",it.peek().unwrap());
                it.next();
                println!("{}",it.peek().unwrap());
                match it.peek().unwrap() {
                    '\'' => {
                        result.push(TokenT::Tchar(ch));
                    }
                    _ => {return Err(format!("Not a char type."));}
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
                match it.peek().unwrap(){
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
                match it.peek().unwrap(){
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
                match it.peek().unwrap(){
                    '=' => {
                        result.push(TokenT::Tnoteq);
                        it.next();
                    }
                    _ => {result.push(TokenT::TlogNot);}
                }
            }
            '<' => {
                it.next();
                match it.peek().unwrap(){
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
                match it.peek().unwrap(){
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
                it.next();
                match it.peek().unwrap(){
                    '.' => {
                        it.next();
                        match it.peek().unwrap(){
                            '.' => {
                                result.push(TokenT::Telipsis);
                                it.next();
                            }
                            _ => {return Err(format!("unexpected character"));}
                        }
                    }
                    _ => {result.push(TokenT::Tdot);}
                }
            }
            '+' => {
                it.next();
                match it.peek().unwrap(){
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
                match it.peek().unwrap(){
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
                match it.peek().unwrap(){
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
            _ => {return Err(format!("unexpected character: {}", c));}
        }
    }
    result.push(TokenT::Teof);
    Ok(result)
}

// pub fn lex_accept()<T: Iterator<Item = TokenT>>(c: char, iter: &mut Peekable<T>, token: TokenT) -> bool {

// }