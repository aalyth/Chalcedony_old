use super::Node;
use crate::lexer::*;
use crate::parser::*;
use super::binary_expression::*;

fn create_condition(tokens: Vec<Token>) -> Box<Node>{
    let binary_expression: bool = tokens
        .iter()
        .any(|token| *token == Token::And || *token == Token::Or);
    if binary_expression {return Box::new(Node::BinaryExpression(generate_binary_expression_tree(&tokens)));}
    return Box::new(Node::from(tokens));
}

#[derive(Debug, Clone)]
pub struct NodeIfStatement{
    condition: Box<Node>,
    body: Vec<Box<Node>>,
    else_statement: Option<Box<Node>>,
}

impl NodeIfStatement{
    fn new() -> Self{
        NodeIfStatement{
            condition: Box::new(Node::None),
            body: Vec::new(),
            else_statement: None,
        }
    }

    pub fn to_c(&self) -> String{
        let mut result: String = "if(".to_owned(); 
        result.push_str(&self.condition.to_c().to_owned());
        result.push_str("){\n");
        for i in &self.body{
            result.push_str(&(&i).to_c()[..]);
            result.push_str(";\n");
        }
        result.push_str("}");
        if !self.else_statement.is_none() {result.push_str(&self.else_statement.as_ref().unwrap().to_c());}
        else {result.push_str("\n");}
        return result;
    }
}

impl From<&Vec<Token>> for NodeIfStatement{
    fn from(tokens: &Vec<Token>) -> Self{
        let mut result = NodeIfStatement::new();
        let mut i = 0;

        if tokens[i] == Token::Keyword(Keyword::If) {i += 1;}
        let mut buffer = Vec::<Token>::new();
        while tokens[i] != Token::Colon{
            buffer.push(tokens[i].clone());
            i += 1;
        }
        //result.condition = Box::new(Node::BinaryExpression(generate_binary_expression_tree(&buffer)));
        //result.condition = Box::new(Node::from(buffer));
        result.condition = create_condition(buffer); 
        i += 1; // skipping over the ':'
        
        // here we basically find the index where this body closes and eventually an
        // elif/else statement starts, so we only split this body while the elif/else statement
        // remains whole
        let else_statement_index = tokens
            .iter()
            .position(|token| *token == Token::Keyword(Keyword::Elif))
            .unwrap_or(tokens
                .iter()
                .position(|token| *token == Token::Keyword(Keyword::Else))
                .unwrap_or(tokens.len()));
        let split_body_tokens = split_tokens(tokens[i .. else_statement_index].to_vec());

        for i in split_body_tokens{
            result.body.push(Box::new(Node::from(i)));
        }

        result.else_statement = match tokens[else_statement_index]{
            Token::Keyword(Keyword::Elif) => Some(Box::new(Node::ElifStatement(NodeElifStatement::from(&tokens[else_statement_index + 1 .. tokens.len()].to_vec())))),
            Token::Keyword(Keyword::Else) => Some(Box::new(Node::ElseStatement(NodeElseStatement::from(&tokens[else_statement_index + 2 .. tokens.len()].to_vec())))),
            _ => None,
        };
        return result;
    }
}

#[derive(Debug, Clone)]
pub struct NodeElifStatement{
    condition: Box<Node>,
    body: Vec<Box<Node>>,
    else_statement: Option<Box<Node>>,
}

impl NodeElifStatement{
    fn new() -> Self{
        NodeElifStatement{
            condition: Box::new(Node::None),
            body: Vec::new(),
            else_statement: None,
        }
    }

    pub fn to_c(&self) -> String{
        let mut result: String = "else if(".to_owned(); 
        result.push_str(&self.condition.to_c().to_owned());
        result.push_str("){\n");
        for i in &self.body{
            result.push_str(&(&i).to_c()[..]);
            result.push_str(";\n");
        }
        result.push_str("}");
        if !self.else_statement.is_none() {result.push_str(&self.else_statement.as_ref().unwrap().to_c());}
        else {result.push_str("\n");}
        return result;
    }
}

impl From<&Vec<Token>> for NodeElifStatement{
    fn from(tokens: &Vec<Token>) -> Self{
        let mut result = NodeElifStatement::new();
        let mut i = 0;

        if tokens[i] == Token::Keyword(Keyword::Elif) {i += 1;}
        let mut buffer = Vec::<Token>::new();
        while tokens[i] != Token::Colon{
            buffer.push(tokens[i].clone());
            i += 1;
        }
        result.condition = Box::new(Node::from(buffer));
        i += 1; // skipping over the ':'
        
        // here we basically find the index where this body closes and eventually an
        // elif/else statement starts, so we only split this body while the elif/else statement
        // remains whole
        let else_statement_index = tokens
            .iter()
            .position(|token| *token == Token::Keyword(Keyword::Elif))
            .unwrap_or(tokens
                .iter()
                .position(|token| *token == Token::Keyword(Keyword::Else))
                .unwrap_or(tokens.len()));
        let split_body_tokens = split_tokens(tokens[i .. else_statement_index].to_vec());

        for i in split_body_tokens{
            result.body.push(Box::new(Node::from(i)));
        }

        result.else_statement = match tokens[else_statement_index]{
            Token::Keyword(Keyword::Elif) => Some(Box::new(Node::ElifStatement(NodeElifStatement::from(&tokens[else_statement_index + 1 .. tokens.len()].to_vec())))),
            Token::Keyword(Keyword::Else) => Some(Box::new(Node::ElseStatement(NodeElseStatement::from(&tokens[else_statement_index + 2 .. tokens.len()].to_vec())))),
            _ => None,
        };
        return result;
    }
}

#[derive(Debug, Clone)]
pub struct NodeElseStatement{
    body: Vec<Box<Node>>,
}

impl NodeElseStatement{
    fn new() -> Self{
        NodeElseStatement{
            body: Vec::new(),
        }
    }

    pub fn to_c(&self) -> String{
        let mut result: String = "else{".to_owned(); 
        for i in &self.body{
            result.push_str(&(&i).to_c()[..]);
            result.push_str(";\n");
        }
        result.push_str("}\n");
        return result;
    }
}

impl From<&Vec<Token>> for NodeElseStatement{
    fn from(tokens: &Vec<Token>) -> Self{
        let mut result = NodeElseStatement::new();
        let end_position = tokens
            .iter()
            .position(|token| *token == Token::Keyword(Keyword::End))
            .unwrap_or(tokens.len());
        let split_body_tokens = split_tokens(tokens[0 .. end_position].to_vec());
        for i in split_body_tokens{
            result.body.push(Box::new(Node::from(i)));
        }
        return result;
    }
}

#[derive(Debug, Clone)]
pub struct NodeWhileLoop{
    condition: Box<Node>,
    body: Vec<Box<Node>>,
}

impl NodeWhileLoop{
    fn new() -> Self{
        NodeWhileLoop{
            condition: Box::new(Node::None),
            body: Vec::new(),
        }
    }

    pub fn to_c(&self) -> String{
        let mut result: String = "while(".to_owned();
        result.push_str(&self.condition.to_c().to_owned());
        result.push_str("){\n");
        for i in &self.body{
            result.push_str(&(&i).to_c()[..]);
            result.push_str(";\n");
        }
        result.push_str("}\n");
        return result;
    }
}

impl From<&Vec<Token>> for NodeWhileLoop{
    fn from(tokens: &Vec<Token>) -> Self{
        let mut result = NodeWhileLoop::new();
        let mut i = 0;

        if tokens[i] == Token::Keyword(Keyword::While) {i += 1;}
        let mut buffer = Vec::<Token>::new();
        while tokens[i] != Token::Colon{
            buffer.push(tokens[i].clone());
            i += 1;
        }
        result.condition = Box::new(Node::from(buffer));
        i += 1; // skipping over the ':'

        let split_body_tokens = split_tokens(tokens[i .. tokens.len() - 1].to_vec());
        for i in split_body_tokens{
            result.body.push(Box::new(Node::from(i)));
        }
        return result;
    }
}
