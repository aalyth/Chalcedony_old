pub mod values;
pub mod variables;
pub mod binary_expression;
pub mod control_flow;
pub mod functions;

use values::*;
use variables::*;
use binary_expression::*;
use control_flow::*;
use functions::*;

use crate::lexer::*;
use crate::parser::*;

#[derive(Debug, Clone)]
pub enum Node{
    ValueInt(NodeValueInt),
    ValueUInt(NodeValueUInt),
    ValueFloat(NodeValueFloat),
    ValueString(NodeValueString),
    VariableCall(NodeVariableCall),
    VariableInitialization(NodeVariableInitialization), 
    VariableDeclaration(NodeVariableDeclaration), // the definition of a variable is a binary expression
    BinaryExpression(NodeBinaryExpression),
    IfStatement(NodeIfStatement), // need to add else handling
    WhileLoop(NodeWhileLoop), 
    FunctionDefinition(NodeFunctionDefinition),
    FunctionCall(NodeFunctionCall),
    None,
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
    // convert this function to 'From<&Token> for Node'
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

    pub fn to_c(&self) -> String{
        match self{
            Node::ValueInt(val)               => return val.to_c(),
            Node::ValueUInt(val)              => return val.to_c(),
            Node::ValueFloat(val)             => return val.to_c(),
            Node::ValueString(val)            => return val.to_c(),
            Node::VariableCall(val)           => return val.to_c(),
            Node::VariableInitialization(val) => return val.to_c(),
            Node::VariableDeclaration(val)    => return val.to_c(),
            Node::BinaryExpression(val)       => return val.to_c(),
            Node::FunctionDefinition(val)     => return val.to_c(),
            Node::FunctionCall(val)           => return val.to_c(),
            _ => todo!(),
        }
    }
}

fn generate_node(tokens: Vec<Token>) -> Node{
    if tokens.len() == 1 {return Node::new(&tokens[0]);}
    match &tokens[0]{
        Token::TokenIdentifier(_val) => return match tokens[1]{
            Token::TokenLPar => Node::FunctionCall(NodeFunctionCall::new(&tokens)),
            _ => Node::BinaryExpression(generate_binary_expression_tree(&tokens)),
        },
        Token::TokenKeyword(_val) => return match &tokens[1]{
            Token::TokenIdentifier(__val) => return generate_variable(&tokens),
            _ => todo!(),
        },
        _ => todo!(),
        //Token::TokenKeyword(Keyword::If) => return 
    }
}
