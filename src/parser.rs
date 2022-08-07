use crate::lexer::Token;
use crate::lexer::Keyword;

macro_rules! get_token_value{
    ($token: expr, $token_type: path) =>{
       match $token{
            $token_type(val) => Some(val),
            _ => None,
        }
    };
}


#[derive(Debug)]
enum VarType{
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
    None,
}

impl From<Keyword> for VarType{
    fn from(keyword: Keyword) -> VarType{
        match keyword{
            Keyword::I8  => return VarType::I8,
            Keyword::I16 => return VarType::I16,
            Keyword::I32 => return VarType::I32,
            Keyword::I64 => return VarType::I64,
            Keyword::U8  => return VarType::U8,
            Keyword::U16 => return VarType::U16,
            Keyword::U32 => return VarType::U32,
            Keyword::U64 => return VarType::U64,
            Keyword::F32 => return VarType::F32,
            Keyword::F64 => return VarType::F64,
            Keyword::Str => return VarType::Str,
            _            => return VarType::None, 
        }
    }
}

#[derive(Debug)]
enum OperatorType{
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
struct NodeValueInt{
    value: i64,
    var_type: VarType,
}

impl NodeValueInt{
    fn new(value: i64, var_type: VarType) -> Self{
        NodeValueInt {value: value, var_type: var_type}
    }
}

#[derive(Debug)]
struct NodeValueUInt{
    value: u64,
    var_type: VarType,
}

impl NodeValueUInt{
    fn new(value: u64, var_type: VarType) -> Self{
        NodeValueUInt {value: value, var_type: var_type}
    }
}

#[derive(Debug)]
struct NodeValueFloat{
    value: f64,
    var_type: VarType,
}

impl NodeValueFloat{
    fn new(value: f64, var_type: VarType) -> Self{
        NodeValueFloat {value: value, var_type: var_type}
    }
}

#[derive(Debug)]
struct NodeValueString{
    value: String,
    var_type: VarType,
}

impl NodeValueString{
    fn new(value: String, var_type: VarType) -> Self{
        NodeValueString {value: value, var_type: var_type}
    }
}

#[derive(Debug)]
struct NodeVariableCall{
    name: String,
}

impl NodeVariableCall{
    fn new(token: &Token) -> Self{
        NodeVariableCall{name: get_token_value!(token, Token::TokenIdentifier).unwrap().to_string()}
    }
}

#[derive(Debug)]
struct NodeVariableInitialization{
    name: String,
    value: Option<Box<Node>>,
    var_type: VarType,
}

#[derive(Debug)]
struct NodeBinaryExpression{
    operands: [Box<Node>;2],
    operator: OperatorType,
}

impl NodeBinaryExpression{
    fn new(tokens: &Vec<Token>) -> Self{
        NodeBinaryExpression {operands: [Box::new(Node::new(&tokens[0])), Box::new(Node::new(&tokens[2]))], operator: OperatorType::from(&tokens[1])}
    }
}

#[derive(Debug)]
struct NodeIfStatement{
    condition: Box<Node>,
    body: Vec<Box<Node>>,
}

#[derive(Debug)]
struct NodeWhileLoop{
    condition: Box<Node>,
    body: Vec<Box<Node>>,
}

#[derive(Debug)]
struct NodeFunctionDefinition{
    name: String,
    arg_names: Vec<String>,
    arg_types: Vec<VarType>,
    return_type: VarType,
    body: Vec<Box<Node>>,
}

#[derive(Debug)]
struct NodeFunctionCall{
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

impl NodeFunctionCall{
    fn new(tokens: &Vec<Token>) -> Self{
        let mut result: NodeFunctionCall = NodeFunctionCall {name: get_token_value!(&tokens[0], Token::TokenIdentifier).unwrap().to_string(), args: Vec::new()};
        for i in 2..(tokens.len() - 1){
            result.args.push(Box::new(get_call(&tokens[i])));
        }
        // if tokens[tokens.len() - 1] != '(' {panic!()} - todo!
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
    VariableInitialization(NodeVariableInitialization), // this handles both declaration and definition
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
}

fn generate_node(tokens: Vec<Token>) -> Node{
    /*
    if tokens[0] == Token::TokenIdentifier{
        if tokens[1] == Token::TokenRPar{
            return NodeFunctionCall::new(tokens);
        }
        // break if tokens.len() > 3
        return NodeBinaryExpression::new(tokens);
    }
    */
    if tokens.len() == 1 {return Node::new(&tokens[0]);}
    match &tokens[0]{
        Token::TokenIdentifier(_val) => return match tokens[1]{
                                                  Token::TokenLPar => Node::FunctionCall(NodeFunctionCall::new(&tokens)),
                                                  _ => Node::BinaryExpression(NodeBinaryExpression::new(&tokens)),
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
