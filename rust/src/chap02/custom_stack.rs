use std::rc::Rc;

use super::stack::Stack;

// 図2.3
pub struct CustomStack<T>(Rc<CustomStackCell<T>>);
enum CustomStackCell<T> {
    Nil,
    Cons(Rc<T>, CustomStack<T>),
}
use CustomStackCell::*;

impl<T> Stack<T> for CustomStack<T> {
    fn empty() -> Self {
        Self(Rc::new(Nil))
    }

    fn is_empty(&self) -> bool {
        match &*self.0 {
            Nil => true,
            _ => false,
        }
    }

    fn head(&self) -> Option<Rc<T>> {
        if let Cons(x, _) = &*self.0 {
            Some(x.clone())
        } else {
            None
        }
    }

    fn cons(&self, x: T) -> Self {
        CustomStack(Rc::new(Cons(Rc::new(x), CustomStack(self.0.clone()))))
    }

    fn tail(&self) -> Option<Self> {
        match &*self.0 {
            Cons(_, t) => Some(CustomStack(t.0.clone())),
            Nil => None,
        }
    }
}
