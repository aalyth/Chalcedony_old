extern crate lazy_static;
extern crate regex;

use crate::lexer::Token;
use crate::lexer::Keyword;
use crate::nodes::Node;
use std::collections::HashMap;
use std::sync::Mutex;

macro_rules! get_token_value{
    ($token: expr, $token_type: path) =>{
       match $token{
            $token_type(val) => Some(val),
            _ => None,
        }
    };
}

pub(crate) use get_token_value;

// here we contain the different variables and their types - this is used
// so we can easily format the print output
lazy_static::lazy_static!{
    static ref VARIABLES: Mutex<HashMap<String, VarType>> = Mutex::new(HashMap::<String, VarType>::new());
}

pub fn variables_insert(string: &String, var_type: &VarType){
    VARIABLES.lock().unwrap().insert(string.to_string(), *var_type);
}

pub fn variables_get(string: &String) -> VarType{
    *VARIABLES.lock().unwrap().get(string).unwrap()
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum VarType{
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Str,
    Auto,
    None,
}

impl From<Keyword> for VarType{
    fn from(keyword: Keyword) -> VarType{
        match keyword{
            Keyword::I8   => return VarType::I8,
            Keyword::I16  => return VarType::I16,
            Keyword::I32  => return VarType::I32,
            Keyword::I64  => return VarType::I64,
            Keyword::U8   => return VarType::U8,
            Keyword::U16  => return VarType::U16,
            Keyword::U32  => return VarType::U32,
            Keyword::U64  => return VarType::U64,
            Keyword::F32  => return VarType::F32,
            Keyword::F64  => return VarType::F64,
            Keyword::Str  => return VarType::Str,
            Keyword::Auto => return VarType::Auto,
            _             => return VarType::None, 
        }
    }
}

impl VarType{
    pub fn new() -> Self{
        VarType::None
    }

    pub fn to_c(&self) -> String{
        match *self{
            VarType::I8   => return "char ".to_string(),
            VarType::I16  => return "int ".to_string(),
            VarType::I32  => return "long ".to_string(),
            VarType::I64  => return "long long ".to_string(),
            VarType::U8   => return "unsigned char ".to_string(),
            VarType::U16  => return "unsigned int ".to_string(),
            VarType::U32  => return "unsigned long ".to_string(),
            VarType::U64  => return "unsigned long long ".to_string(),
            VarType::F32  => return "float ".to_string(),
            VarType::F64  => return "double ".to_string(),
            VarType::Str  => return "str ".to_string(),
            VarType::None => return "void ".to_string(),
            VarType::Auto => todo!(),
        }
    }

    pub fn to_c_printf(&self) -> String{
        match *self{
            VarType::I8   => return "%hd".to_string(),
            VarType::I16  => return "%d".to_string(),
            VarType::I32  => return "%ld".to_string(),
            VarType::I64  => return "%lld".to_string(),
            VarType::U8   => return "%hu".to_string(),
            VarType::U16  => return "%u".to_string(),
            VarType::U32  => return "%lu".to_string(),
            VarType::U64  => return "%llu".to_string(),
            VarType::F32  => return "%f".to_string(),
            VarType::F64  => return "%lf".to_string(),
            VarType::Str  => return "%s".to_string(),
            _ => todo!(),
        }
    }

    pub fn to_c_default_value(&self) -> String{
        match *self{
            VarType::I8   => return "0".to_string(),
            VarType::I16  => return "0".to_string(),
            VarType::I32  => return "0".to_string(),
            VarType::I64  => return "0".to_string(),
            VarType::U8   => return "0".to_string(),
            VarType::U16  => return "0".to_string(),
            VarType::U32  => return "0".to_string(),
            VarType::U64  => return "0".to_string(),
            VarType::F32  => return "0.0f".to_string(),
            VarType::F64  => return "0.0".to_string(),
            VarType::Str  => return "\"\"".to_string(),
            _ => todo!(),
        }
    }
}
 
#[derive(Debug, Clone)]
pub enum OperatorType{
    Plus,
    Minus,
    Mul,
    Div,
    FloorDiv,
    Exp,
    Eq,
    EqEq,
    NEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    PlusEq,
    MinusEq,
    MulEq,
    DivEq,
    // todo!() - ExpEq (**=), FloorDivEq ( //=)
    None,
}

impl From<&Token> for OperatorType{
    fn from(token: &Token) -> OperatorType{
        match *token{
            Token::Plus     => return OperatorType::Plus,
            Token::Minus    => return OperatorType::Minus,
            Token::Mul      => return OperatorType::Mul,
            Token::Div      => return OperatorType::Div,
            Token::FloorDiv => return OperatorType::FloorDiv,
            Token::Exp      => return OperatorType::Exp,
            Token::Eq       => return OperatorType::Eq,
            Token::EqEq     => return OperatorType::EqEq,
            Token::NEq      => return OperatorType::NEq,
            Token::Lt       => return OperatorType::Lt,
            Token::Gt       => return OperatorType::Gt,
            Token::LtEq     => return OperatorType::LtEq,
            Token::GtEq     => return OperatorType::GtEq,
            Token::PlusEq   => return OperatorType::PlusEq,
            Token::MinusEq  => return OperatorType::MinusEq,
            Token::MulEq    => return OperatorType::MulEq,
            Token::DivEq    => return OperatorType::DivEq,
            _               => return OperatorType::None,
        }
    }
}

impl OperatorType{
    pub fn new() -> Self{
        OperatorType::None
    }

    pub fn to_c(&self) -> String{
        match *self{
            OperatorType::Plus       => return "+ ".to_string(),
            OperatorType::Minus      => return "- ".to_string(),
            OperatorType::Mul        => return "* ".to_string(),
            OperatorType::Div        => return "/ ".to_string(),
            //OperatorType::FloorDiv => return "+ ".to_string(),
            //OperatorType::Exp      => return "+ ".to_string(),
            OperatorType::Eq         => return "= ".to_string(),
            OperatorType::EqEq       => return "== ".to_string(),
            OperatorType::NEq        => return "!= ".to_string(),
            OperatorType::Lt         => return "< ".to_string(),
            OperatorType::Gt         => return "> ".to_string(),
            OperatorType::LtEq       => return "<= ".to_string(),
            OperatorType::GtEq       => return ">= ".to_string(),
            OperatorType::PlusEq     => return "+= ".to_string(),
            OperatorType::MinusEq    => return "-= ".to_string(),
            OperatorType::MulEq      => return "*= ".to_string(),
            OperatorType::DivEq      => return "/= ".to_string(),
            _ => return "".to_string(),
        }
    }
}

pub fn split_tokens(tokens: Vec<Token>) -> Vec<Vec<Token>>{
    let mut openings: u8; // how many blocks we are in
    let mut current: Vec<Token>;
    let mut result: Vec<Vec<Token>> = Vec::new();
    let mut i = 0;
    while i < tokens.len() - 1{
        openings = 0;
        current = Vec::new();
        loop{
            match tokens[i]{
                Token::Keyword(Keyword::If)    => openings += 1,       
                Token::Keyword(Keyword::While) => openings += 1,
                Token::Keyword(Keyword::For)   => openings += 1,
                Token::Keyword(Keyword::Fn)   => openings += 1,
                Token::Keyword(Keyword::End)   => openings -= 1,
                _ => (),
            }
            current.push(tokens[i].clone());
            i += 1;
            if openings == 0 && tokens[i] == Token::NewLine {break;}
        }
        result.push(current);
    }
    return result;
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node>{
    let token_blocks = split_tokens(tokens);
    let mut result: Vec<Node> = Vec::new();
    for i in token_blocks{ 
        //result.push(generate_function(&tokens[i..].to_vec()));
        result.push(Node::from(i));
    }
    return result;
}
