use crate::lexer::*;
use crate::parser::*;
use super::Node;
use crate::nodes::*;

#[derive(Debug, Clone)]
pub struct NodeFunctionDefinition{
    name: String,
    arg_names: Vec<String>,
    arg_types: Vec<VarType>,
    return_type: VarType,
    body: Vec<Box<Node>>,
}

impl NodeFunctionDefinition{
    pub fn to_c(&self) -> String{
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
            result.push_str(";\n");
        }
        result.push_str("}\n");
        return result;
    }
}

#[derive(Debug, Clone)]
pub struct NodeFunctionCall{
    name: String,
    args: Vec<Box<Node>>,
}

pub fn get_call(token: &Token) -> Node{
    match token{
        Token::Int8(val)    => return Node::ValueInt(NodeValueInt::new(*val as i64, VarType::I8)),
        Token::Int16(val)   => return Node::ValueInt(NodeValueInt::new(*val as i64, VarType::I16)),
        Token::Int32(val)   => return Node::ValueInt(NodeValueInt::new(*val as i64, VarType::I32)),
        Token::Int64(val)   => return Node::ValueInt(NodeValueInt::new(*val as i64, VarType::I64)),
        Token::UInt8(val)   => return Node::ValueUInt(NodeValueUInt::new(*val as u64, VarType::U8)),
        Token::UInt16(val)  => return Node::ValueUInt(NodeValueUInt::new(*val as u64, VarType::U16)),
        Token::UInt32(val)  => return Node::ValueUInt(NodeValueUInt::new(*val as u64, VarType::U32)),
        Token::UInt64(val)  => return Node::ValueUInt(NodeValueUInt::new(*val as u64, VarType::U64)),
        Token::Float32(val) => return Node::ValueFloat(NodeValueFloat::new(*val as f64, VarType::F32)),
        Token::Float64(val) => return Node::ValueFloat(NodeValueFloat::new(*val as f64, VarType::F64)),
        Token::String(val)  => return Node::ValueString(NodeValueString::new(val.to_string(), VarType::Str)),
        _ => return Node::VariableCall(NodeVariableCall::from(token)),
    }
}

pub fn format_printf(string: String) -> Vec<Box<Node>>{
    let formatting = regex::Regex::new(r"(?:[^\\])(\{\w+[^\\\}]\})").unwrap();    
    let mut result = string.to_owned();
    let mut vec_result = Vec::<Box<Node>>::new();

    for matches in formatting.captures_iter(&string){
        let var_name: String = (&matches[0][2..matches[0].len()-1]).to_string();
        vec_result.push(Box::new(Node::VariableCall(NodeVariableCall::from(&Token::Identifier(var_name.to_owned())))));
        result = str::replace(&result, &matches[0][1..], &VarType::to_c_printf(&variables_get(&var_name)));
    }

    vec_result.insert(0, Box::new(Node::ValueString(NodeValueString::new(result, VarType::Str))));
    return vec_result;
}

impl NodeFunctionCall{
    pub fn new(tokens: &Vec<Token>) -> Self{
        let mut result: NodeFunctionCall = NodeFunctionCall {name: get_token_value!(&tokens[0], Token::Identifier).unwrap().to_string(), args: Vec::new()};
        for i in 2..(tokens.len() - 1){
            result.args.push(Box::new(get_call(&tokens[i])));
        }
        // if tokens[tokens.len() - 1] != '(' {panic!()} - todo!
        return result;
    }

    pub fn to_c(&self) -> String{
        let mut result: String = self.name.to_owned();
        if self.name == "print" {
            result.push_str("f(");
            let new_args: Vec<Box<Node>> = format_printf(self.args[0].to_c());
            for i in 0..new_args.len(){
                result.push_str(&new_args[i].to_c().to_owned());
                if i != new_args.len() - 1 {result.push_str(", ");}
            }

            result.push_str(")");
            return result;
        }

        result.push_str("(");
        for i in 0..self.args.len(){
            result.push_str(&self.args[i].to_c().to_owned());
            if i != self.args.len() - 1 {result.push_str(", ");}
        }

        result.push_str(")");
        return result;
    }
}

pub fn generate_function(tokens: &Vec<Token>) -> Node{
    println!("function tokens = {:#?}\n", tokens);
    let mut i = 0;
    let mut result: NodeFunctionDefinition = NodeFunctionDefinition {
        name: "".to_string(), 
        arg_names: Vec::new(), 
        arg_types: Vec::new(),
        body: Vec::new(), 
        return_type: VarType::None
    };

    i += 1; 
    result.name = get_token_value!(&tokens[i], Token::Identifier).unwrap().to_string();
    i += 2; // this is so we skip the opening bracket - '('

    while tokens[i] != Token::RPar{
        result.arg_names.push(get_token_value!(&tokens[i], Token::Identifier).unwrap().to_string());
        i += 2;
        result.arg_types.push(VarType::from(get_token_value!(tokens[i], Token::Keyword).unwrap()));
        i += 1;
        variables_insert(&result.arg_names[result.arg_names.len() - 1], &result.arg_types[result.arg_types.len() - 1]);
    }

    i += 2; // here we skip the ')' and '=>' 
    result.return_type = VarType::from(get_token_value!(tokens[i], Token::Keyword).unwrap()); 
    i += 1;
    if tokens[i] == Token::Colon {i += 2;}

    while tokens[i] != Token::Keyword(Keyword::End){
        let mut buffer: Vec<Token> = Vec::new();
        while tokens[i] != Token::NewLine{
            buffer.push(tokens[i].clone());
            i += 1;
        }

        while tokens[i] == Token::NewLine { i += 1; }
        if buffer.len() != 0 {println!("buffer = {:#?}\n", buffer); result.body.push(Box::new(Node::from(buffer)));}
    }
    return Node::FunctionDefinition(result);
}

