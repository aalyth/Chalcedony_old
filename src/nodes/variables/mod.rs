use crate::lexer::Token;
use crate::parser::*;
use super::Node;

use crate::nodes::{generate_binary_expression_tree, evaluate_var_type};

#[derive(Debug, Clone)]
pub struct NodeVariableCall{
    name: String,
}

impl NodeVariableCall{
    pub fn new(token: &Token) -> Self{
        NodeVariableCall{name: get_token_value!(token, Token::TokenIdentifier).unwrap().to_string()}
    }

    pub fn to_c(&self) -> String{
        self.name.to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct NodeVariableInitialization{
    name: String,
    value: Box<Node>,
    var_type: VarType,
}

impl NodeVariableInitialization{
    pub fn new(tokens: &Vec<Token>) -> Self{
        let mut result = NodeVariableInitialization {
            name: get_token_value!(&tokens[1], Token::TokenIdentifier).unwrap().to_string(), 
            value: Box::new(Node::None),
            var_type: VarType::from(*(get_token_value!(&tokens[0], Token::TokenKeyword).unwrap()))
        };

        if tokens.len() > 4 {result.value = Box::new(Node::BinaryExpression(generate_binary_expression_tree(&tokens[3..].to_vec())));}
        else {result.value = Box::new(Node::new(&tokens[3]));}

        if result.var_type == VarType::Auto {
            result.var_type = evaluate_var_type(&result.value);
        }

        variables_insert(&result.name, &result.var_type);
        return result;
    }

    pub fn to_c(&self) -> String{
       let mut result: String = self.var_type.to_c().to_owned();
       result.push_str(&self.name.to_owned());
       result.push_str(" = ");
       result.push_str(&self.value.to_c().to_owned());
       return result;
    }
}

#[derive(Debug, Clone)]
pub struct NodeVariableDeclaration{
    name: String,
    var_type: VarType,
}

impl NodeVariableDeclaration{
    pub fn new(tokens: &Vec<Token>) -> Self{
        let result = NodeVariableDeclaration {
            name: get_token_value!(&tokens[1], Token::TokenIdentifier).unwrap().to_string(), 
            var_type: VarType::from(*(get_token_value!(&tokens[0], Token::TokenKeyword).unwrap()))
        };
        variables_insert(&result.name, &result.var_type);
        return result;
    }

    pub fn to_c(&self) -> String{
       let mut result: String = self.var_type.to_c().to_owned();
       result.push_str(&self.name.to_owned());
       result.push_str(" = ");
       result.push_str(&self.var_type.to_c_default_value().to_owned());
       return result;
    }
}

pub fn generate_variable(tokens: &Vec<Token>) -> Node{
    // if tokens.len() < 3 panic
    if tokens.len() < 3 {Node::VariableDeclaration(NodeVariableDeclaration::new(tokens))}
    else{Node::VariableInitialization(NodeVariableInitialization::new(tokens))}
}
