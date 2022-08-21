use std::collections::VecDeque;

#[derive(Debug)]
pub struct Stack<T>{
    values: VecDeque<T>,
}

impl<T> Stack<T>{
    pub fn new() -> Self{
        Stack {values: VecDeque::<T>::new()}
    }

    pub fn insert(&mut self, value: T){
        self.values.push_back(value);
    }

    pub fn pop(&mut self) -> T{
        self.values.pop_back().unwrap()
    }
    
    pub fn empty(&self) -> bool{
        self.values.is_empty()
    }

    pub fn top(&self) -> &T{
        self.values.get(self.values.len() - 1).unwrap()
    }
}
