// 図2.3

use std::{fmt::Debug, rc::Rc};

use crate::common::stack::Stack;

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

    fn head(&self) -> Result<T, String> {
        Ok(self.get()?.0)
    }

    fn tail(&self) -> Result<Self, String> {
        Ok(self.get()?.1)
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

    pub fn get(&self) -> Result<(T, Self), String> {
        if let Cons(x, t) = &*self.0 {
            Ok((x.clone(), t.clone()))
        } else {
            Err("empty stack.".to_string())
        }
    }

    pub fn map<U: Clone, F: Fn(T) -> U>(&self, f: F) -> CustomStack<U> {
        match &*self.0 {
            Nil => CustomStack::empty(),
            Cons(x, t) => {
                let y = (&f)(x.clone());
                t.map(f).cons(y)
            }
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

    #[test]
    fn test_get() {
        println!("{:?}", CustomStack::<u32>::empty().get());
        println!("{:?}", CustomStack::empty().cons(0).get());
    }

    #[test]
    fn test_map() {
        let stack: CustomStack<_> = CustomStack::empty().cons(4).cons(3).cons(2).cons(1).cons(0);
        println!("{:?}", stack.map(|x| x + 100));
    }
}
