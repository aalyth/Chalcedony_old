extern crate regex;

#[derive(Debug, Copy, Clone)]
pub enum KEYWORD{
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

#[derive(Debug)]
pub enum TOKEN{
    TokenInt8(i8),
    TokenInt16(i16),
    TokenInt32(i32),
    TokenInt64(i64),
    TokenUint8(u8),
    TokenUint16(u16),
    TokenUint32(u32),
    TokenUint64(u64),
    TokenFloat32(f32),
    TokenFloat64(f64),
    TokenString(String),
    TokenPlus, // +
    TokenMinus, // -
    TokenMul, // *
    TokenDiv, // /
    TokenFloorDiv, // //
    TokenExp, // **
    TokenLPar, // (
    TokenRpar, // )
    TokenEq, // =
    TokenEqEq, // ==
    TokenNEq, // !=
    TokenLt, // <
    TokenGt, // >
    TokenLtEq, // <=
    TokenGtEq, // >=
    TokenColon, // :
    TokenReturn, // =>
    TokenNewLine, // \n
    TokenKeyword(KEYWORD),
    TokenIdentifier(String)
    /*
    TOKEN_CHAR, // to add string token
    TOKEN_FUNC,
    TOKEN_LET,
    TOKEN_VARNAME
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

fn to_digit(s: &str) -> TOKEN{
    let _result: i64 = s.parse().unwrap();
    if _result < 0{
        match _result{
            -128                       ..= 0              => return TOKEN::TokenInt8(_result as i8),
            -32_768                    ..= -129           => return TOKEN::TokenInt16(_result as i16),
            -2_147_483_648             ..= -32_769        => return TOKEN::TokenInt32(_result as i32),
            _ => return TOKEN::TokenInt64(_result as i64),
        }
    }else{
        let _resut: u64 = _result as u64;        
        match _result{
            0             ..=  255                        => return TOKEN::TokenUint8(_result as u8), 
            256           ..=  65_535                     => return TOKEN::TokenUint16(_result as u16),
            65_536        ..=  4_294_967_295              => return TOKEN::TokenUint32(_result as u32),
            _ => return TOKEN::TokenUint64(_result as u64),
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

fn to_float(s: &str) -> TOKEN{
    let result: f64 = s.parse().unwrap(); 
    match result{
        x if x >= -3.40282347E+38 && x <= 3.40282347E+38 => return TOKEN::TokenFloat32(result as f32),
        _ => return TOKEN::TokenFloat64(result), 
    }
}

pub fn lexer(src_code: &str) -> Vec<TOKEN>{
    println!("{}", src_code);
    let lines = src_code.split('\n');
    let mut result = Vec::<TOKEN>::new();
    let keywords = std::collections::HashMap::from([
        ("auto", KEYWORD::Auto),
        ("none", KEYWORD::None),
        ("i8", KEYWORD::I8),
        ("i16", KEYWORD::I16),
        ("i32", KEYWORD::I32),
        ("i64", KEYWORD::I64),
        ("u8", KEYWORD::U8),
        ("u16", KEYWORD::U16),
        ("u32", KEYWORD::U32),
        ("u64", KEYWORD::U64),
        ("f32", KEYWORD::F32),
        ("f64", KEYWORD::F64),
        ("and", KEYWORD::And),
        ("or", KEYWORD::Or),
        ("not", KEYWORD::Not),
        ("if", KEYWORD::If),
        ("elif", KEYWORD::Elif),
        ("for", KEYWORD::For),
        ("to", KEYWORD::To),
        ("while", KEYWORD::While),
        ("end", KEYWORD::End),
        ("fn", KEYWORD::Fn),
        ("return", KEYWORD::Return),
    ]);
    let re = regex::Regex::new(r#"(\n)|("[a-zA-Z0-9 ]+")|(=>)|(!=)|(==)|([a-zA-Z0-9\-_]+)|[\(\):=\+\-\*/<>\#]"#).unwrap();

    for line in lines{
        //let tokens: Vec<&str> = i.split(' ').collect();

        //let j: &str = &k.replace("\t", "");
        for matches in re.captures_iter(line){
            let token: &str = &matches[0];
    //            println!("{}", j);
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
                result.push(TOKEN::TokenString(token.to_string()));
                continue;
            }

            if keywords.contains_key(token){
                result.push(TOKEN::TokenKeyword(*keywords.get(token).unwrap())); 
                continue;
            }

            match token{
                "+"  => result.push(TOKEN::TokenPlus),
                "-"  => result.push(TOKEN::TokenMinus),
                "*"  => result.push(TOKEN::TokenMul),
                "/"  => result.push(TOKEN::TokenDiv),
                "//" => result.push(TOKEN::TokenFloorDiv),
                "**" => result.push(TOKEN::TokenExp),
                "("  => result.push(TOKEN::TokenLPar),
                ")"  => result.push(TOKEN::TokenRpar),
                "="  => result.push(TOKEN::TokenEq),
                "==" => result.push(TOKEN::TokenEqEq),
                "!=" => result.push(TOKEN::TokenNEq),
                "<"  => result.push(TOKEN::TokenLt),
                ">"  => result.push(TOKEN::TokenGt),
                "<=" => result.push(TOKEN::TokenLtEq),
                ">=" => result.push(TOKEN::TokenGtEq),
                ":"  => result.push(TOKEN::TokenColon),
                "=>" => result.push(TOKEN::TokenReturn),
                _    => result.push(TOKEN::TokenIdentifier(token.to_string())),
            }
        }
        result.push(TOKEN::TokenNewLine);
    }
    return result;
}
