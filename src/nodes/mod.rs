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
    fn new() -> Self{
        Node::None
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

impl From<&Token> for Node{
    fn from(token: &Token) -> Node{
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
            Token::Identifier(_val) => return Node::VariableCall(NodeVariableCall::from(token)),
            _ => todo!(),
        }
    }
}

impl From<Vec<Token>> for Node{
    fn from(tokens: Vec<Token>) -> Node{
        println!("from tokens = {:#?}\n", tokens);
        if tokens.len() == 1 {return Node::from(&tokens[0]);}
        match &tokens[0]{
            Token::Keyword(Keyword::I8)  => return generate_variable(&tokens),
            Token::Keyword(Keyword::I16) => return generate_variable(&tokens),
            Token::Keyword(Keyword::I32) => return generate_variable(&tokens),
            Token::Keyword(Keyword::I64) => return generate_variable(&tokens),
            Token::Keyword(Keyword::U8)  => return generate_variable(&tokens),
            Token::Keyword(Keyword::U16) => return generate_variable(&tokens),
            Token::Keyword(Keyword::U32) => return generate_variable(&tokens),
            Token::Keyword(Keyword::U64) => return generate_variable(&tokens),
            Token::Keyword(Keyword::F32) => return generate_variable(&tokens),
            Token::Keyword(Keyword::F64) => return generate_variable(&tokens),
            Token::Keyword(Keyword::Str) => return generate_variable(&tokens),
            Token::Keyword(Keyword::Fn)  => return generate_function(&tokens), 
            Token::Identifier(_val) => return match tokens[1]{
                Token::LPar => return Node::FunctionCall(NodeFunctionCall::new(&tokens)),
                _           => return Node::BinaryExpression(generate_binary_expression_tree(&tokens)),
            },
            _ => todo!()
        }
    }
}

