use crate::lexer::Token;
use crate::parser::VarType;

#[derive(Debug)]
pub struct NodeValueInt{
        value: i64,
        var_type: VarType,
}

impl NodeValueInt{
    pub fn new(value: i64, var_type: VarType) -> Self{
        NodeValueInt {value: value, var_type: var_type}
    }

    pub fn to_c(&self) -> String{
        self.value.to_string()
    }
}

#[derive(Debug)]
pub struct NodeValueUInt{
    value: u64,
    var_type: VarType,
}

impl NodeValueUInt{
    pub fn new(value: u64, var_type: VarType) -> Self{
        NodeValueUInt {value: value, var_type: var_type}
    }

    pub fn to_c(&self) -> String{
        self.value.to_string()
    }
}

#[derive(Debug)]
pub struct NodeValueFloat{
    value: f64,
    var_type: VarType,
}

impl NodeValueFloat{
    pub fn new(value: f64, var_type: VarType) -> Self{
        NodeValueFloat {value: value, var_type: var_type}
    }

    pub fn to_c(&self) -> String{
        self.value.to_string()
    }
}

#[derive(Debug)]
pub struct NodeValueString{
    value: String,
    var_type: VarType,
}

impl NodeValueString{
    pub fn new(value: String, var_type: VarType) -> Self{
        NodeValueString {value: value, var_type: var_type}
    }

    pub fn to_c(&self) -> String{
        self.value.to_owned()
    }
}


