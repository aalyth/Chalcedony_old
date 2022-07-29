mod lexer;

enum VarType{
    i8,
    i16,
    i32,
    i64,
    u8,
    u16,
    u32,
    u64,
    f32,
    f64,
}

enum OperatorType{
    Plus,
    Minus,
    Mult,
    Div,
    FloorDiv,
    Exp
}

enum NodeType<T>{
    Value{
        value: T,
    },
    VariableInitialization{ // this handles both declaration and definition
        name: &str,
        value: Option<Box<NodeType<T>>>,
        type: VarType,
    }, 
    BinaryOperation{
        operands: [T;2], 
        operator: OperatorType,
        type: VarType,
    },
    IfStatement{
        condition: Option<Box<NodeType<T>::BinaryOperation>>,
        body: Vec<Option<Box<NodeType<T>>>>,
        else: Option<Box<NodeType<T>>>,
    },
    ForLoop{
        condition: Option<Box<NodeType<T>::BinaryOperation>>,
        body: Vec<Option<Box<NodeType<T>>>>,
        init_var: Option<Box<NodeType<T>::VariableInitialization>>, // this always creates a
                                                                    // variable with value of 0/1
        end_val: Option<Box<NodeType<T>::Value>>, // this is to where we cycle
    },
    WhileLoop{
        condition: Option<Box<NodeType<T>::BinaryOperation>>,
        body: Vec<Option<Box<NodeType<T>>>>,
    },
    FunctionDeclaration{
        name: &str,
        return_type: VarType,
        arg_names: Vec<&str>,
        arg_types: Vec<VarType>,
        body: Option<Box<NodeType<T>>>,
        return: Option<Box<NodeType<T>::Value>>,
    },
    FunctionCall{
        name: 
        arg_vals: Vec<Option<Box<NodeType<T>>>,
    },
}
