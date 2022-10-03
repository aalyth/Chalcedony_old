extern crate regex;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Keyword{
    Auto,
    None,
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
    If,
    Elif,
    Else,
    For,
    To,
    While,
    End,
    Fn,
    Return
    // Continue and Break 
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token{
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    // to add TokenChar
    String(String),
    Plus,     // +
    Minus,    // -
    Mul,      // *
    Div,      // /
    Mod,      // %
    FloorDiv, // //
    Exp,      // **
    LPar,     // (
    RPar,     // )
    Eq,       // =
    EqEq,     // ==
    NotEq,    // !=
    Lt,       // <
    Gt,       // >
    LtEq,     // <=
    GtEq,     // >=
    PlusEq,   // +=
    MinusEq,  // -=
    MulEq,    // *=
    DivEq,    // /=
    ModEq,    // %=
    Colon,    // :
    Return,   // ->
    NewLine,  // \n
    And,      // &&
    Or,       // ||
    Not,      // !        |
    // Reference // &     | unary operators
    // Dereference // *   |
    Keyword(Keyword),
    Identifier(String)
    /*
    Token_CHAR, // to add string token
    */
}

fn is_digit(s: &str) -> bool{
    if s.len() == 1 && s.chars().nth(0).unwrap() == '-' { return false;}
    for i in s.chars(){
        match i{
            '0'..='9' => continue,
            '-' => continue,
            _ => return false
        }
    } 
    return true;
}

fn to_digit(s: &str) -> Token{
    let _result: i64 = s.parse().unwrap();
    if _result < 0{
        match _result{
            -128           ..= 0       => return Token::Int8(_result as i8),
            -32_768        ..= -129    => return Token::Int16(_result as i16),
            -2_147_483_648 ..= -32_769 => return Token::Int32(_result as i32),
            _ => return Token::Int64(_result as i64),
        }
    }else{
        let _resut: u64 = _result as u64;        
        match _result{
            0      ..= 255           => return Token::UInt8(_result as u8), 
            256    ..= 65_535        => return Token::UInt16(_result as u16),
            65_536 ..= 4_294_967_295 => return Token::UInt32(_result as u32),
            _ => return Token::UInt64(_result as u64),
        }
    }
}

fn is_float(s: &str) -> bool{
    let mut has_dot: bool = false;
    for i in s.chars(){
        match i{
            '0'..='9' => continue,
            '-' => continue,
            '.' => has_dot = true,
            _ => return false
        }
    } 
    return has_dot;
}

fn to_float(s: &str) -> Token{
    let result: f64 = s.parse().unwrap(); 
    match result{
        x if x >= -3.40282347E+38 && x <= 3.40282347E+38 => return Token::Float32(result as f32),
        _ => return Token::Float64(result), 
    }
}

pub fn lexer(src_code: &str) -> Vec<Token>{
    let lines = src_code.split('\n');
    let mut result = Vec::<Token>::new();
    let keywords = std::collections::HashMap::from([
        ("auto", Keyword::Auto),
        ("none", Keyword::None),
        ("i8",   Keyword::I8),
        ("i16",  Keyword::I16),
        ("i32", Keyword::I32),
        ("i64", Keyword::I64),
        ("u8", Keyword::U8),
        ("u16", Keyword::U16),
        ("u32", Keyword::U32),
        ("u64", Keyword::U64),
        ("f32", Keyword::F32),
        ("f64", Keyword::F64),
        ("str", Keyword::Str),
        ("if", Keyword::If),
        ("elif", Keyword::Elif),
        ("else", Keyword::Else),
        ("for", Keyword::For),
        ("to", Keyword::To),
        ("while", Keyword::While),
        ("end", Keyword::End),
        ("fn", Keyword::Fn),
        ("return", Keyword::Return),
    ]);

    let re = regex::Regex::new(r#"(#.*)|(\n)|(".+")|(\*\*)|(//)|(->)|(&&)|(\|\|)|([!&\*])|([=!<>\+\-\*/%]=)|(\d+(\.\d*)*)|([a-zA-Z0-9\-_]+)|[\(\):=\+\-\*/<>\#%]"#).unwrap();

    for line in lines{
        for matches in re.captures_iter(line){
            let token: &str = &matches[0];
            if token.is_empty() {continue;}
            if token.chars().nth(0).unwrap() == '#'{
                break; 
            }

            if is_digit(token){
                result.push(to_digit(token)); 
                continue;
            }

            if is_float(token){
                result.push(to_float(token));
                continue;
            }


            if token.chars().nth(0).unwrap() == '"' && token.chars().nth(token.len() - 1).unwrap() == '"'{
                result.push(Token::String(token.to_string()));
                continue;
            }

            if keywords.contains_key(token){
                result.push(Token::Keyword(*keywords.get(token).unwrap())); 
                continue;
            }

            match token{
                "+"  => result.push(Token::Plus),
                "-"  => result.push(Token::Minus),
                "*"  => result.push(Token::Mul),
                "/"  => result.push(Token::Div),
                "%"  => result.push(Token::Mod),
                "//" => result.push(Token::FloorDiv),
                "**" => result.push(Token::Exp),
                "("  => result.push(Token::LPar),
                ")"  => result.push(Token::RPar),
                "="  => result.push(Token::Eq),
                "==" => result.push(Token::EqEq),
                "!=" => result.push(Token::NotEq),
                "<"  => result.push(Token::Lt),
                ">"  => result.push(Token::Gt),
                "<=" => result.push(Token::LtEq),
                ">=" => result.push(Token::GtEq),
                "+=" => result.push(Token::PlusEq),
                "-=" => result.push(Token::MinusEq),
                "*=" => result.push(Token::MulEq),
                "/=" => result.push(Token::DivEq),
                "%=" => result.push(Token::ModEq),
                ":"  => result.push(Token::Colon),
                "->" => result.push(Token::Return),
                "&&" => result.push(Token::And),
                "||" => result.push(Token::Or),
                "!"  => result.push(Token::Not),
                _    => result.push(Token::Identifier(token.to_string())),
            }
        }
        result.push(Token::NewLine);
    }
    return result;
}
