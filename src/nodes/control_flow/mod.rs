use crate::lexer::Token;
use super::Node;

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
