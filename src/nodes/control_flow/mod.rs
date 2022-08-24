use super::Node;

#[derive(Debug, Clone)]
pub struct NodeIfStatement{
    condition: Box<Node>,
    body: Vec<Box<Node>>,
}

#[derive(Debug, Clone)]
pub struct NodeWhileLoop{
    condition: Box<Node>,
    body: Vec<Box<Node>>,
}
