use crate::lexer::*;
use crate::parser::*;
use super::Node;

#[derive(Debug, Clone)]
pub struct NodeUnaryExpression{
    operand: Box<Node>,
    operator: OperatorType,
}

impl NodeUnaryExpression{
    fn new() -> Self{
        NodeUnaryExpression{
            operand: Box::new(Node::new()),
            operator: OperatorType::new(),
        }
    }

    pub fn to_c(&self) -> String{
        let mut result: String = self.operator.to_c().to_owned();
        result.push_str("(");
        result.push_str(&self.operand.to_c().to_owned());
        result.push_str(")");
        return result;
    }
}

impl From<&Vec<Token>> for NodeUnaryExpression{
    fn from(tokens: &Vec<Token>) -> Self{
        NodeUnaryExpression{
            operand: Box::new(Node::from(tokens[1..].to_vec())),
            operator: OperatorType::from(&tokens[0]),
        }
    }
}
