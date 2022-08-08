extern crate lazy_static;
extern crate regex;

use crate::lexer::Token;
use crate::lexer::Keyword;
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

// here we contain the different variables and their types - this is used
// so we can easily format the print output
lazy_static::lazy_static!{
    static ref VARIABLES: Mutex<HashMap<String, VarType>> = Mutex::new(HashMap::<String, VarType>::new());
}

fn variables_insert(string: &String, var_type: &VarType){
    VARIABLES.lock().unwrap().insert(string.to_string(), *var_type);
}

fn variables_get(string: &String) -> VarType{
    *VARIABLES.lock().unwrap().get(string).unwrap()
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
    fn to_c(&self) -> String{
        match *self{
            VarType::I8   => return "short ".to_string(),
            VarType::I16  => return "int ".to_string(),
            VarType::I32  => return "long ".to_string(),
            VarType::I64  => return "long long ".to_string(),
            VarType::U8   => return "unsigned short ".to_string(),
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

    fn to_c_printf(&self) -> String{
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
}
 
#[derive(Debug)]
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
            _                    => return OperatorType::None,
        }
    }
}

#[derive(Debug)]
pub struct NodeValueInt{
    value: i64,
    var_type: VarType,
}

impl NodeValueInt{
    fn new(value: i64, var_type: VarType) -> Self{
        NodeValueInt {value: value, var_type: var_type}
    }

    fn to_c(&self) -> String{
        self.value.to_string()
    }
}

#[derive(Debug)]
pub struct NodeValueUInt{
    value: u64,
    var_type: VarType,
}

impl NodeValueUInt{
    fn new(value: u64, var_type: VarType) -> Self{
        NodeValueUInt {value: value, var_type: var_type}
    }

    fn to_c(&self) -> String{
        self.value.to_string()
    }
}

#[derive(Debug)]
pub struct NodeValueFloat{
    value: f64,
    var_type: VarType,
}

impl NodeValueFloat{
    fn new(value: f64, var_type: VarType) -> Self{
        NodeValueFloat {value: value, var_type: var_type}
    }

    fn to_c(&self) -> String{
        self.value.to_string()
    }
}

#[derive(Debug)]
pub struct NodeValueString{
    value: String,
    var_type: VarType,
}

impl NodeValueString{
    fn new(value: String, var_type: VarType) -> Self{
        NodeValueString {value: value, var_type: var_type}
    }

    fn to_c(&self) -> String{
        self.value.to_owned()
    }
}

#[derive(Debug)]
pub struct NodeVariableCall{
    name: String,
}

impl NodeVariableCall{
    fn new(token: &Token) -> Self{
        NodeVariableCall{name: get_token_value!(token, Token::TokenIdentifier).unwrap().to_string()}
    }

    fn to_c(&self) -> String{
        self.name.to_owned()
    }
}

#[derive(Debug)]
pub struct NodeVariableInitialization{
    name: String,
    value: Box<Node>,
    var_type: VarType,
}

impl NodeVariableInitialization{
    fn new(tokens: &Vec<Token>) -> Self{
        let mut result = NodeVariableInitialization {
            name: get_token_value!(&tokens[1], Token::TokenIdentifier).unwrap().to_string(), 
            value: Box::new(Node::new(&tokens[3])), // this is always a node to a value 
            var_type: VarType::from(*(get_token_value!(&tokens[0], Token::TokenKeyword).unwrap()))
        };
        if result.var_type == VarType::Auto {
            result.var_type = match &tokens[3]{
                Token::TokenInt8(_val)    => VarType::I8,
                Token::TokenInt16(_val)   => VarType::I16,
                Token::TokenInt32(_val)   => VarType::I32,
                Token::TokenInt64(_val)   => VarType::I64,
                Token::TokenUInt8(_val)   => VarType::U8,
                Token::TokenUInt16(_val)  => VarType::U16,
                Token::TokenUInt32(_val)  => VarType::U32,
                Token::TokenUInt64(_val)  => VarType::U64,
                Token::TokenFloat32(_val) => VarType::F32,
                Token::TokenFloat64(_val) => VarType::F64,
                Token::TokenString(_val)  => VarType::Str,
                _ => VarType::Auto,
            };
        }
        variables_insert(&result.name, &result.var_type);
        return result;
    }

    fn to_c(&self) -> String{
       let mut result: String = self.var_type.to_c().to_owned();
       result.push_str(&self.name.to_owned());
       result.push_str(" = ");
       result.push_str(&self.value.to_c().to_owned());
       result.push_str(";\n");
       return result;
    }
}

#[derive(Debug)]
pub struct NodeVariableDeclaration{
    name: String,
    var_type: VarType,
}

impl NodeVariableDeclaration{
    fn new(tokens: &Vec<Token>) -> Self{
        let result = NodeVariableDeclaration {
            name: get_token_value!(&tokens[1], Token::TokenIdentifier).unwrap().to_string(), 
            var_type: VarType::from(*(get_token_value!(&tokens[0], Token::TokenKeyword).unwrap()))
        };
        variables_insert(&result.name, &result.var_type);
        return result;
    }

    fn to_c(&self) -> String{
       let mut result: String = self.var_type.to_c().to_owned();
       result.push_str(&self.name.to_owned());
       result.push_str(";\n");
       return result;
    }
}

fn generate_variable(tokens: &Vec<Token>) -> Node{
    // if tokens.len() < 3 panic
    match tokens[2]{
        Token::TokenEq => return Node::VariableInitialization(NodeVariableInitialization::new(tokens)),
        Token::TokenNewLine => return Node::VariableDeclaration(NodeVariableDeclaration::new(tokens)),
        _ => todo!(),
    }
}

#[derive(Debug)]
pub struct NodeBinaryExpression{
    operands: [Box<Node>;2],
    operator: OperatorType,
}

impl NodeBinaryExpression{
    fn new(tokens: &Vec<Token>) -> Self{
        NodeBinaryExpression {operands: [Box::new(Node::new(&tokens[0])), Box::new(Node::new(&tokens[2]))], operator: OperatorType::from(&tokens[1])}
    }
}

#[derive(Debug)]
pub struct NodeIfStatement{
    condition: Box<Node>,
    body: Vec<Box<Node>>,
}

#[derive(Debug)]
pub struct NodeWhileLoop{
    condition: Box<Node>,
    body: Vec<Box<Node>>,
}

#[derive(Debug)]
pub struct NodeFunctionDefinition{
    name: String,
    arg_names: Vec<String>,
    arg_types: Vec<VarType>,
    return_type: VarType,
    body: Vec<Box<Node>>,
}

impl NodeFunctionDefinition{
    fn to_c(&self) -> String{
        let mut result: String = "".to_string().to_owned();
        result.push_str(&self.return_type.to_c()[..]);
        result.push_str(&self.name[..]);
        result.push_str("(");

        for i in 0..self.arg_names.len(){
            result.push_str(&self.arg_types[i].to_c()[..]);
            result.push_str(&self.arg_names[i][..]);
            if i != self.arg_names.len() - 1 {result.push_str(", ");}
        }
        result.push_str("){\n");

        for i in &self.body{
            result.push_str(&(&i).to_c()[..]);
        }
        result.push_str("}\n");
        return result;
    }
}

#[derive(Debug)]
pub struct NodeFunctionCall{
    name: String,
    args: Vec<Box<Node>>,
}

fn get_call(token: &Token) -> Node{
    match token{
        Token::TokenInt8(val)    => return Node::ValueInt(NodeValueInt::new(*val as i64, VarType::I8)),
        Token::TokenInt16(val)   => return Node::ValueInt(NodeValueInt::new(*val as i64, VarType::I16)),
        Token::TokenInt32(val)   => return Node::ValueInt(NodeValueInt::new(*val as i64, VarType::I32)),
        Token::TokenInt64(val)   => return Node::ValueInt(NodeValueInt::new(*val as i64, VarType::I64)),
        Token::TokenUInt8(val)   => return Node::ValueUInt(NodeValueUInt::new(*val as u64, VarType::U8)),
        Token::TokenUInt16(val)  => return Node::ValueUInt(NodeValueUInt::new(*val as u64, VarType::U16)),
        Token::TokenUInt32(val)  => return Node::ValueUInt(NodeValueUInt::new(*val as u64, VarType::U32)),
        Token::TokenUInt64(val)  => return Node::ValueUInt(NodeValueUInt::new(*val as u64, VarType::U64)),
        Token::TokenFloat32(val) => return Node::ValueFloat(NodeValueFloat::new(*val as f64, VarType::F32)),
        Token::TokenFloat64(val) => return Node::ValueFloat(NodeValueFloat::new(*val as f64, VarType::F64)),
        Token::TokenString(val)  => return Node::ValueString(NodeValueString::new(val.to_string(), VarType::Str)),
        _ => return Node::VariableCall(NodeVariableCall::new(token)),
    }
}

fn format_printf(string: String) -> Vec<Box<Node>>{
    let formatting = regex::Regex::new(r"(?:[^\\])(\{\w+[^\\\}]\})").unwrap();    
    let mut result = string.to_owned();
    let mut vec_result = Vec::<Box<Node>>::new();
    for matches in formatting.captures_iter(&string){
        let var_name: String = (&matches[0][2..matches[0].len()-1]).to_string();
        vec_result.push(Box::new(Node::VariableCall(NodeVariableCall::new(&Token::TokenIdentifier(var_name.to_owned())))));
        result = str::replace(&result, &matches[0][1..], &VarType::to_c_printf(&variables_get(&var_name)));
    }
    vec_result.insert(0, Box::new(Node::ValueString(NodeValueString::new(result, VarType::Str))));
    return vec_result;
}

impl NodeFunctionCall{
    fn new(tokens: &Vec<Token>) -> Self{
        let mut result: NodeFunctionCall = NodeFunctionCall {name: get_token_value!(&tokens[0], Token::TokenIdentifier).unwrap().to_string(), args: Vec::new()};
        for i in 2..(tokens.len() - 1){
            result.args.push(Box::new(get_call(&tokens[i])));
        }
        // if tokens[tokens.len() - 1] != '(' {panic!()} - todo!
        return result;
    }

    fn to_c(&self) -> String{
        let mut result: String = self.name.to_owned();
        if self.name == "print" {
            result.push_str("f(");
            let new_args: Vec<Box<Node>> = format_printf(self.args[0].to_c());
            for i in 0..new_args.len(){
                result.push_str(&new_args[i].to_c().to_owned());
                if i != new_args.len() - 1 {result.push_str(", ");}
            }
            result.push_str(");\n");
            return result;
        }

        result.push_str("(");
        for i in 0..self.args.len(){
            result.push_str(&self.args[i].to_c().to_owned());
            if i != self.args.len() - 1 {result.push_str(", ");}
        }
        result.push_str(");\n");
        return result;
    }
}

#[derive(Debug)]
pub enum Node{
    ValueInt(NodeValueInt),
    ValueUInt(NodeValueUInt),
    ValueFloat(NodeValueFloat),
    ValueString(NodeValueString),
    VariableCall(NodeVariableCall),
    VariableInitialization(NodeVariableInitialization), 
    VariableDeclaration(NodeVariableDeclaration), // the definition of a variable is a binary expression
    BinaryExpression(NodeBinaryExpression),
    IfStatement(NodeIfStatement), // need to add else handling
    WhileLoop(NodeWhileLoop), 
    FunctionDefinition(NodeFunctionDefinition),
    FunctionCall(NodeFunctionCall),
    /*
    ForLoop{
        condition: Option<Box<NodeType<T>::BinaryOperation>>,
        body: Vec<Option<Box<NodeType<T>>>>,
        init_var: Option<Box<NodeType<T>::VariableInitialization>>, // this always creates a
                                                                    // variable with value of 0/1
        end_val: Option<Box<NodeType<T>::Value>>, // this is to where we cycle
    },
    */
}

impl Node{
    fn new(token: &Token) -> Self{
        match token{
            Token::TokenInt8(val)    => return Node::ValueInt(NodeValueInt::new(*val as i64, VarType::I8)),
            Token::TokenInt16(val)   => return Node::ValueInt(NodeValueInt::new(*val as i64, VarType::I16)),
            Token::TokenInt32(val)   => return Node::ValueInt(NodeValueInt::new(*val as i64, VarType::I32)),
            Token::TokenInt64(val)   => return Node::ValueInt(NodeValueInt::new(*val as i64, VarType::I64)),
            Token::TokenUInt8(val)   => return Node::ValueUInt(NodeValueUInt::new(*val as u64, VarType::U8)),
            Token::TokenUInt16(val)  => return Node::ValueUInt(NodeValueUInt::new(*val as u64, VarType::U16)),
            Token::TokenUInt32(val)  => return Node::ValueUInt(NodeValueUInt::new(*val as u64, VarType::U32)),
            Token::TokenUInt64(val)  => return Node::ValueUInt(NodeValueUInt::new(*val as u64, VarType::U64)),
            Token::TokenFloat32(val) => return Node::ValueFloat(NodeValueFloat::new(*val as f64, VarType::F32)),
            Token::TokenFloat64(val) => return Node::ValueFloat(NodeValueFloat::new(*val as f64, VarType::F64)),
            Token::TokenString(val)  => return Node::ValueString(NodeValueString::new(val.to_string(), VarType::Str)),
            Token::TokenIdentifier(_val) => return Node::VariableCall(NodeVariableCall::new(token)),
            _ => todo!(),
        }
    }

    pub fn to_c(&self) -> String{
        match self{
            Node::ValueInt(val)               => return val.to_c(),
            Node::ValueUInt(val)              => return val.to_c(),
            Node::ValueFloat(val)             => return val.to_c(),
            Node::ValueString(val)            => return val.to_c(),
            Node::VariableCall(val)           => return val.to_c(),
            Node::VariableInitialization(val) => return val.to_c(),
            Node::VariableDeclaration(val)    => return val.to_c(),
            Node::FunctionDefinition(val)     => return val.to_c(),
            Node::FunctionCall(val)           => return val.to_c(),
            _ => todo!(),
        }
    }
}

fn generate_node(tokens: Vec<Token>) -> Node{
    if tokens.len() == 1 {return Node::new(&tokens[0]);}
    match &tokens[0]{
        Token::TokenIdentifier(_val) => return match tokens[1]{
            Token::TokenLPar => Node::FunctionCall(NodeFunctionCall::new(&tokens)),
            _ => Node::BinaryExpression(NodeBinaryExpression::new(&tokens)),
        },
        Token::TokenKeyword(_val) => return match &tokens[1]{
            Token::TokenIdentifier(__val) => return generate_variable(&tokens),
            _ => todo!(),
        },
        _ => todo!(),
        //Token::TokenKeyword(Keyword::If) => return 
    }
}

fn generate_function(tokens: &Vec<Token>) -> Node{
    let mut i = 0;
    while tokens[i] != Token::TokenKeyword(Keyword::Fn) {i += 1;}
    let mut result: NodeFunctionDefinition = NodeFunctionDefinition {name: "".to_string(), arg_names: Vec::new(), arg_types: Vec::new(), body: Vec::new(), return_type: VarType::None};
    //result.name = match tokens[++i] {Token::TokenIdentifier(s) => s, _ => ()};
    i += 1; 
    result.name = get_token_value!(&tokens[i], Token::TokenIdentifier).unwrap().to_string();
    i += 2; // this is so we skip the opening bracket - '('
    while tokens[i] != Token::TokenRPar{
        result.arg_names.push(get_token_value!(&tokens[i], Token::TokenIdentifier).unwrap().to_string());
        i += 2;
        result.arg_types.push(VarType::from(get_token_value!(tokens[i], Token::TokenKeyword).unwrap()));
    }
    i += 2; // here we skip the ')' and '=>' 
    result.return_type = VarType::from(get_token_value!(tokens[i], Token::TokenKeyword).unwrap()); 
    i += 1;
    if tokens[i] == Token::TokenColon {i += 2;}
    while tokens[i] != Token::TokenKeyword(Keyword::End){
        let mut buffer: Vec<Token> = Vec::new();
        while tokens[i] != Token::TokenNewLine{
            buffer.push(tokens[i].clone());
            i += 1;
        }
        i += 1; // so we skip the newline
        result.body.push(Box::new(generate_node(buffer))); 
    }
    return Node::FunctionDefinition(result);
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
