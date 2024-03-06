// 図2.3

use std::{fmt::Debug, rc::Rc};

use super::super::common::stack::Stack;

pub struct CustomStack<T>(Rc<CustomStackCell<T>>);
enum CustomStackCell<T> {
    Nil,
    Cons(T, CustomStack<T>),
}
use CustomStackCell::*;

impl<T> Clone for CustomStack<T> {
    fn clone(&self) -> Self {
        CustomStack(self.0.clone())
    }
}

impl<T: Debug> Debug for CustomStack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self.0 {
            Nil => write!(f, "[]"),
            Cons(x, t) => write!(f, "({:?}) :: {:?}", x, t),
        }
    }
}

impl<T: Clone> Stack<T> for CustomStack<T> {
    fn empty() -> Self {
        CustomStack(Rc::new(Nil))
    }

    fn is_empty(&self) -> bool {
        match &*self.0 {
            Nil => true,
            _ => false,
        }
    }

    fn cons(&self, x: T) -> Self {
        CustomStack(Rc::new(Cons(x, CustomStack(self.0.clone()))))
    }

    fn head(&self) -> T {
        if let Cons(x, _) = &*self.0 {
            x.clone()
        } else {
            panic!("empty stream.")
        }
    }

    fn tail(&self) -> Self {
        match &*self.0 {
            Cons(_, t) => CustomStack(t.0.clone()),
            Nil => panic!("empty stream."),
        }
    }
}

impl<T: Clone> CustomStack<T> {
    // 演習問題 2.1
    fn suffixes(&self) -> CustomStack<CustomStack<T>> {
        match &*self.0 {
            Nil => CustomStack::empty().cons(self.clone()),
            Cons(_, t) => t.suffixes().cons(self.clone()),
        }
    }
}

mod test {
    use super::CustomStack;
    use crate::common::stack::Stack;

    #[test]
    fn test_suffixes() {
        let l = CustomStack::empty().cons(4).cons(3).cons(2).cons(1u32);
        println!("{:?}", l.suffixes());
    }
}
