use crate::lexer::*;
use crate::parser::*;
use super::Node;
use crate::stack::Stack;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NodeBinaryExpression{
    operands: [Box<Node>;2],
    operator: OperatorType,
}

impl NodeBinaryExpression{
    pub fn new() -> Self{
        NodeBinaryExpression {operands: [Box::new(Node::None), Box::new(Node::None)], operator: OperatorType::None}
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

impl Token{
    fn precedence(&self) -> u8{
        match *self{
            Token::TokenPlus     => 1,
            Token::TokenPlusEq   => 1,
            Token::TokenMinus    => 1,
            Token::TokenMinusEq  => 1,
            Token::TokenMul      => 2,
            Token::TokenMulEq    => 2,
            Token::TokenDiv      => 2,
            Token::TokenDivEq    => 2,
            Token::TokenExp      => 3,
            Token::TokenFloorDiv => 3,
            Token::TokenRPar     => 0,
            _ => 0
        }
    }
}

impl From<&Vec<Token>> for NodeBinaryExpression{
    fn from(tokens: &Vec<Token>) -> Self{
        NodeBinaryExpression {operands: [Box::new(Node::new(&tokens[0])), Box::new(Node::new(&tokens[2]))], operator: OperatorType::from(&tokens[1])}
    }
}

fn is_arithmetic(token: &Token) -> bool{
    // returns true if this is a keyword or a non-string value
    match token{
        Token::TokenIdentifier(_val) => return true,
        Token::TokenInt8(_val)    => return true,
        Token::TokenInt16(_val)   => return true,
        Token::TokenInt32(_val)   => return true,
        Token::TokenInt64(_val)   => return true,
        Token::TokenUInt8(_val)   => return true,
        Token::TokenUInt16(_val)  => return true,
        Token::TokenUInt32(_val)  => return true,
        Token::TokenUInt64(_val)  => return true,
        Token::TokenFloat32(_val) => return true,
        Token::TokenFloat64(_val) => return true,
        // Token::TokenString(_val)  => return true,
        _ => return false,
    }
}

// todo()! => to make auto keyword solve the whole binary expression tree to find out variable type
// an adaptation of the Shunting-yard algorithm for infix notation
pub fn generate_binary_expression_tree(tokens: &Vec<Token>) -> NodeBinaryExpression{
    let mut st_c: Stack<Token> = Stack::<Token>::new(); // character (Token) stack 
    let mut st_n: Stack<Node> = Stack::<Node>::new(); // node stack
    for i in tokens{
        if *i == Token::TokenLPar {
            st_c.insert(i.clone());

        }else if is_arithmetic(i){
            st_n.insert(Node::new(i));

        }else if i.precedence() > 0{
            while !st_c.empty() &&
                  *st_c.top() != Token::TokenLPar &&
                  ((*i != Token::TokenExp && st_c.top().precedence() >= i.precedence()) ||
                  (*i == Token::TokenExp && st_c.top().precedence() > i.precedence())){
                
                let operator: OperatorType = OperatorType::from(&st_c.pop());
                let operand2: Node = st_n.pop();
                let operand1: Node = st_n.pop();
                let mut temp: NodeBinaryExpression = NodeBinaryExpression::new();

                temp.operands[0] = Box::new(operand1);
                temp.operands[1] = Box::new(operand2);
                temp.operator = operator;
                st_n.insert(Node::BinaryExpression(temp));
            }
            st_c.insert(i.clone());

        }else if *i == Token::TokenRPar{
            while !st_c.empty() && *st_c.top() != Token::TokenLPar {
                let operator: OperatorType = OperatorType::from(&st_c.pop());
                let operand2: Node = st_n.pop();
                let operand1: Node = st_n.pop();
                let mut temp: NodeBinaryExpression = NodeBinaryExpression::new();

                temp.operands[0] = Box::new(operand1);
                temp.operands[1] = Box::new(operand2);
                temp.operator = operator;
                st_n.insert(Node::BinaryExpression(temp));
            }
 
            let _remove = st_c.pop();
        }
    }

    while !st_c.empty() && *st_c.top() != Token::TokenLPar {
        let operator: OperatorType = OperatorType::from(&st_c.pop());
        let operand2: Node = st_n.pop();
        let operand1: Node = st_n.pop();
        let mut temp: NodeBinaryExpression = NodeBinaryExpression::new();

        temp.operands[0] = Box::new(operand1);
        temp.operands[1] = Box::new(operand2);
        temp.operator = operator;
        st_n.insert(Node::BinaryExpression(temp));
    }

    let result: NodeBinaryExpression = match st_n.top(){
        Node::BinaryExpression(val) => val.clone(),
        _ => NodeBinaryExpression::new(),
    };
    return result;
}

fn greater_var_type(type1: &VarType, type2: &VarType) -> VarType{
    let values: HashMap<VarType, i8> = HashMap::from([
        (VarType::None, 0),
        (VarType::I8, 1),
        (VarType::I16, 3),
        (VarType::I32, 5),
        (VarType::I64, 8),
        (VarType::U8, 2),
        (VarType::U16, 4),
        (VarType::U32, 6),
        (VarType::U64, 9),
        (VarType::F32, 7),
        (VarType::F64, 10),
    ]);
    if values.get(type1) > values.get(type2){
        return type1.clone();
    }
    return type2.clone();
}

pub fn evaluate_var_type(root: &Node) -> VarType{
    if let Node::BinaryExpression(val) = root{
        let left = evaluate_var_type(&val.operands[0]);
        let right = evaluate_var_type(&val.operands[1]);
        return greater_var_type(&left, &right);
    }
    match root{
        Node::ValueInt(val) => return val.var_type,
        Node::ValueUInt(val) => return val.var_type,
        Node::ValueFloat(val) => return val.var_type,
        Node::ValueString(val) => return val.var_type,
        _ => return VarType::None,
    }
}
