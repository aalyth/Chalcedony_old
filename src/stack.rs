use std::collections::LinkedList;

pub struct Stack<T>{
    values: LinkedList<T>,
}

impl<T> Stack<T>{
    pub fn insert(&mut self, value: T){
        self.values.push_back(value);
    }

    pub fn pop(&mut self) -> Option<T>{
        self.values.pop_back()
    }
}
