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
    And,
    Or,
    Not,
    If,
    Elif,
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
    TokenInt8(i8),
    TokenInt16(i16),
    TokenInt32(i32),
    TokenInt64(i64),
    TokenUInt8(u8),
    TokenUInt16(u16),
    TokenUInt32(u32),
    TokenUInt64(u64),
    TokenFloat32(f32),
    TokenFloat64(f64),
    // to add TokenChar
    TokenString(String),
    TokenPlus,     // +
    TokenMinus,    // -
    TokenMul,      // *
    TokenDiv,      // /
    TokenFloorDiv, // //
    TokenExp,      // **
    TokenLPar,     // (
    TokenRPar,     // )
    TokenEq,       // =
    TokenEqEq,     // ==
    TokenNEq,      // !=
    TokenLt,       // <
    TokenGt,       // >
    TokenLtEq,     // <=
    TokenGtEq,     // >=
    TokenPlusEq,   // +=
    TokenMinusEq,  // -=
    TokenMulEq,    // *=
    TokenDivEq,    // /=
    TokenColon,    // :
    TokenReturn,   // =>
    TokenNewLine,  // \n
    TokenKeyword(Keyword),
    TokenIdentifier(String)
    /*
    Token_CHAR, // to add string token
    */
}

fn is_digit(s: &str) -> bool{
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
            -128           ..= 0       => return Token::TokenInt8(_result as i8),
            -32_768        ..= -129    => return Token::TokenInt16(_result as i16),
            -2_147_483_648 ..= -32_769 => return Token::TokenInt32(_result as i32),
            _ => return Token::TokenInt64(_result as i64),
        }
    }else{
        let _resut: u64 = _result as u64;        
        match _result{
            0      ..= 255           => return Token::TokenUInt8(_result as u8), 
            256    ..= 65_535        => return Token::TokenUInt16(_result as u16),
            65_536 ..= 4_294_967_295 => return Token::TokenUInt32(_result as u32),
            _ => return Token::TokenUInt64(_result as u64),
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
        x if x >= -3.40282347E+38 && x <= 3.40282347E+38 => return Token::TokenFloat32(result as f32),
        _ => return Token::TokenFloat64(result), 
    }
}

pub fn lexer(src_code: &str) -> Vec<Token>{
    println!("{}", src_code);
    let lines = src_code.split('\n');
    let mut result = Vec::<Token>::new();
    let keywords = std::collections::HashMap::from([
        ("auto", Keyword::Auto),
        ("none", Keyword::None),
        ("i8", Keyword::I8),
        ("i16", Keyword::I16),
        ("i32", Keyword::I32),
        ("i64", Keyword::I64),
        ("u8", Keyword::U8),
        ("u16", Keyword::U16),
        ("u32", Keyword::U32),
        ("u64", Keyword::U64),
        ("f32", Keyword::F32),
        ("f64", Keyword::F64),
        ("and", Keyword::And),
        ("or", Keyword::Or),
        ("not", Keyword::Not),
        ("if", Keyword::If),
        ("elif", Keyword::Elif),
        ("for", Keyword::For),
        ("to", Keyword::To),
        ("while", Keyword::While),
        ("end", Keyword::End),
        ("fn", Keyword::Fn),
        ("return", Keyword::Return),
    ]);

    let re = regex::Regex::new(r#"(#.*)|(\n)|(".+")|(\*\*)|(//)|(->)|([=!<>\+\-\*/]=)|(\d+(\.\d*)*)|([a-zA-Z0-9\-_]+)|[\(\):=\+\-\*/<>\#]"#).unwrap();

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
                result.push(Token::TokenString(token.to_string()));
                continue;
            }

            if keywords.contains_key(token){
                result.push(Token::TokenKeyword(*keywords.get(token).unwrap())); 
                continue;
            }

            match token{
                "+"  => result.push(Token::TokenPlus),
                "-"  => result.push(Token::TokenMinus),
                "*"  => result.push(Token::TokenMul),
                "/"  => result.push(Token::TokenDiv),
                "//" => result.push(Token::TokenFloorDiv),
                "**" => result.push(Token::TokenExp),
                "("  => result.push(Token::TokenLPar),
                ")"  => result.push(Token::TokenRPar),
                "="  => result.push(Token::TokenEq),
                "==" => result.push(Token::TokenEqEq),
                "!=" => result.push(Token::TokenNEq),
                "<"  => result.push(Token::TokenLt),
                ">"  => result.push(Token::TokenGt),
                "<=" => result.push(Token::TokenLtEq),
                ">=" => result.push(Token::TokenGtEq),
                "+=" => result.push(Token::TokenPlusEq),
                "-=" => result.push(Token::TokenMinusEq),
                "*=" => result.push(Token::TokenMulEq),
                "/=" => result.push(Token::TokenDivEq),
                ":"  => result.push(Token::TokenColon),
                "->" => result.push(Token::TokenReturn),
                _    => result.push(Token::TokenIdentifier(token.to_string())),
            }
        }
        result.push(Token::TokenNewLine);
    }
    return result;
}
