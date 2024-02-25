use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

pub enum Suspension<T> {
    Fun(Box<dyn FnOnce() -> T>),
    Val(Rc<T>),
}
use Suspension::*;

impl<T> Suspension<T> {
    pub fn new(f: Box<dyn FnOnce() -> T>) -> Self {
        Fun(f)
    }

    pub fn get(&mut self) -> Rc<T> {
        match self {
            Val(x) => x.clone(),
            Fun(r) => {
                let f = std::mem::replace(r, Box::new(|| panic!()));
                let x = Rc::new(f());
                *self = Val(x.clone());
                x
            }
        }
    }
}

impl<T: Debug> Debug for Suspension<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fun(_) => {
                write!(f, "LazyFun")
            }
            Val(x) => {
                write!(f, "{:?}", x)
            }
        }
    }
}

impl<T: Display> Display for Suspension<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fun(_) => {
                write!(f, "LazyFun")
            }
            Val(x) => {
                write!(f, "{}", x)
            }
        }
    }
}
