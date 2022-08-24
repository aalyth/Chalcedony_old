use crate::lexer::*;
use crate::parser::*;
use super::Node;

use crate::nodes::{generate_binary_expression_tree, evaluate_var_type};

#[derive(Debug, Clone)]
pub struct NodeVariableCall{
    name: String,
}

impl NodeVariableCall{
    pub fn new() -> Self{
        NodeVariableCall{
            name: "".to_owned()
        } 
    }

    pub fn to_c(&self) -> String{
        self.name.to_owned()
    }
}

impl From<&Token> for NodeVariableCall{
    fn from(token: &Token) -> Self{
        NodeVariableCall{name: get_token_value!(token, Token::Identifier).unwrap().to_string()}
    }
}

#[derive(Debug, Clone)]
pub struct NodeVariableInitialization{
    name: String,
    value: Box<Node>,
    var_type: VarType,
}

impl NodeVariableInitialization{
    pub fn new() -> Self{
        NodeVariableInitialization {
            name: "".to_owned(),
            value: Box::new(Node::new()),
            var_type: VarType::new(),
        }
    }

    pub fn to_c(&self) -> String{
       let mut result: String = self.var_type.to_c().to_owned();
       result.push_str(&self.name.to_owned());
       result.push_str(" = ");
       result.push_str(&self.value.to_c().to_owned());
       return result;
    }
}

impl From<&Vec<Token>> for NodeVariableInitialization{
    fn from(tokens: &Vec<Token>) -> Self{
        let mut result = NodeVariableInitialization {
            name: get_token_value!(&tokens[1], Token::Identifier).unwrap().to_string(), 
            value: Box::new(Node::None),
            var_type: VarType::from(*(get_token_value!(&tokens[0], Token::Keyword).unwrap()))
        };

        if tokens.len() > 4 {result.value = Box::new(Node::BinaryExpression(generate_binary_expression_tree(&tokens[3..].to_vec())));}
        else {result.value = Box::new(Node::from(&tokens[3]));}

        if result.var_type == VarType::Auto {
            result.var_type = evaluate_var_type(&result.value);
        }

        variables_insert(&result.name, &result.var_type);
        return result;
    }
}

#[derive(Debug, Clone)]
pub struct NodeVariableDeclaration{
    name: String,
    var_type: VarType,
}

impl NodeVariableDeclaration{
    pub fn new() -> Self{
        NodeVariableDeclaration {
            name: "".to_owned(),
            var_type: VarType::new(), 
        }
    }

    pub fn to_c(&self) -> String{
       let mut result: String = self.var_type.to_c().to_owned();
       result.push_str(&self.name.to_owned());
       result.push_str(" = ");
       result.push_str(&self.var_type.to_c_default_value().to_owned());
       return result;
    }
}

impl From<&Vec<Token>> for NodeVariableDeclaration{
    fn from(tokens: &Vec<Token>) -> Self{
        let result = NodeVariableDeclaration {
            name: get_token_value!(&tokens[1], Token::Identifier).unwrap().to_string(), 
            var_type: VarType::from(*(get_token_value!(&tokens[0], Token::Keyword).unwrap()))
        };
        variables_insert(&result.name, &result.var_type);
        return result;
    }

}

pub fn generate_variable(tokens: &Vec<Token>) -> Node{
    // if tokens.len() < 3 panic
    if tokens.len() < 3 {Node::VariableDeclaration(NodeVariableDeclaration::from(tokens))}
    else{Node::VariableInitialization(NodeVariableInitialization::from(tokens))}
}
