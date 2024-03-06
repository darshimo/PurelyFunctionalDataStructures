// 図2.3

use std::rc::Rc;

use super::stack::Stack;

pub struct CustomStack<T>(Rc<CustomStackCell<T>>);
enum CustomStackCell<T> {
    Nil,
    Cons(T, CustomStack<T>),
}
use CustomStackCell::*;

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
