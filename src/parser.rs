extern crate lazy_static;
extern crate regex;

use crate::nodes::Node;
use crate::nodes::functions::generate_function;

use crate::stack::Stack;
use crate::lexer::Token;
use crate::lexer::Keyword;
use std::collections::HashMap;
use std::sync::Mutex;

#[macro_use]
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
            Token::TokenPlus     => return OperatorType::Plus,
            Token::TokenMinus    => return OperatorType::Minus,
            Token::TokenMul      => return OperatorType::Mul,
            Token::TokenDiv      => return OperatorType::Div,
            Token::TokenFloorDiv => return OperatorType::FloorDiv,
            Token::TokenExp      => return OperatorType::Exp,
            Token::TokenEq       => return OperatorType::Eq,
            Token::TokenEqEq     => return OperatorType::EqEq,
            Token::TokenNEq      => return OperatorType::NEq,
            Token::TokenLt       => return OperatorType::Lt,
            Token::TokenGt       => return OperatorType::Gt,
            Token::TokenLtEq     => return OperatorType::LtEq,
            Token::TokenGtEq     => return OperatorType::GtEq,
            Token::TokenPlusEq   => return OperatorType::PlusEq,
            Token::TokenMinusEq  => return OperatorType::MinusEq,
            Token::TokenMulEq    => return OperatorType::MulEq,
            Token::TokenDivEq    => return OperatorType::DivEq,
            _                    => return OperatorType::None,
        }
    }
}

impl OperatorType{
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

/*
fn split_tokens(tokens: Vec<Tokens>) -> Vec<Vec<&Tokens>>{
    let mut endings_left: i64 = -1; // this counts how many blocks we are in
    let mut result = Vec::new();
    let mut current: Vec<&Tokens> = Vec::new();
    for i in tokens{
        if i == Token::TokenKeyword(Keyword::Fn){
            endings_left = 1;
            continue;
        }

        match i{
            Token::TokenKeyword(Keyword::If) => endings_left += 1,
            Token::TokenKeyword(Keyword::While) => endings_left += 1,
            Token::TokenKeyword(Keyword::For) => endings_left += 1,
            Token::TokenKeyword(Keyword::End) => endings_left -= 1,
        }


        if endings_left == 0{
            
        }
    }
}
*/

pub fn parse(tokens: Vec<Token>) -> Vec<Node>{
    let mut result: Vec<Node> = Vec::new();
    for i in 0..tokens.len(){
        if tokens[i] == Token::TokenKeyword(Keyword::Fn){
            result.push(generate_function(&tokens[i..].to_vec()));
        }
    }
    return result;
}
