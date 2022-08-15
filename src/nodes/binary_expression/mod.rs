use crate::lexer::*;
use crate::parser::*;
use super::Node;

#[derive(Debug)]
pub struct NodeBinaryExpression{
    operands: [Box<Node>;2],
    operator: OperatorType,
}

impl NodeBinaryExpression{
    pub fn new(tokens: &Vec<Token>) -> Self{
        NodeBinaryExpression {operands: [Box::new(Node::new(&tokens[0])), Box::new(Node::new(&tokens[2]))], operator: OperatorType::from(&tokens[1])}
    }

    pub fn to_c(&self) -> String{
        let mut result: String = "(".to_owned();
        result.push_str(&self.operands[0].to_c().to_owned());
        result.push_str(" ");
        result.push_str(&self.operator.to_c().to_owned());
        result.push_str(&self.operands[1].to_c().to_owned());
        result.push_str(")");
        return result;
    }
}

// var2 = 
// (15 + 5) * 3
pub fn generate_binary_expression_tree(tokens: &Vec<Token>) -> NodeBinaryExpression{
    println!("tokens = {:#?}\n", tokens);
    let mut result: NodeBinaryExpression = NodeBinaryExpression {
        operands: [Box::new(Node::None), Box::new(Node::None)],
        operator: OperatorType::None,
    };

    if tokens[0] == Token::TokenLPar{
        let mut left_side: Vec<Token> = Vec::new();
        let mut scopes = 1u64; // the number of brackets entered 
        for i in 1..tokens.len() - 3{
            if tokens[i] == Token::TokenRPar {
                scopes -= 1;
                if scopes == 0 {
                    if tokens.len() - i > 1 {
                        result.operands[0] = Box::new(Node::BinaryExpression(generate_binary_expression_tree(&left_side)));
                        result.operator = OperatorType::from(&tokens[i - 2]);
                        result.operands[1] = Box::new(Node::BinaryExpression(generate_binary_expression_tree(&tokens[i+2..].to_vec())));
                        return result;
                    } 
                    break;
                }
            }
            if tokens[i] == Token::TokenLPar {scopes += 1;}
            left_side.push(tokens[i].clone());
        }
        result.operands[0] = Box::new(Node::BinaryExpression(generate_binary_expression_tree(&left_side)));
        result.operator = OperatorType::from(&tokens[tokens.len() - 2]);
        result.operands[1] = Box::new(Node::new(&tokens[tokens.len() - 1]));
        return result;
    }

    result = NodeBinaryExpression::new(tokens); 
    if tokens.len() > 3{
        result.operands[1] = Box::new(Node::BinaryExpression(generate_binary_expression_tree(&tokens[2..].to_vec())));
    }
    return result;
}
