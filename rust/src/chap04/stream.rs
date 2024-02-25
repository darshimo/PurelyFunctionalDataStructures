use super::{stack::Stack, suspension::Susp};
use std::{fmt::Display, rc::Rc};

struct Stream<T>(Susp<StreamCell<T>>);

enum StreamCell<T> {
    Nil,
    Cons(Rc<T>, Stream<T>),
}
use StreamCell::*;

impl<T: Display> Display for StreamCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Nil => write!(f, "[]"),
            Cons(x, t) => write!(f, "({}) :: {}", x, t),
        }
    }
}

impl<T> Clone for Stream<T> {
    fn clone(&self) -> Self {
        Stream(self.0.clone())
    }
}

impl<T: Display> Display for Stream<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: 'static> Stack<T> for Stream<T> {
    fn empty() -> Self {
        Stream(Susp::new(Box::new(|| Nil)))
    }

    fn is_empty(&self) -> bool {
        match &*self.0.get() {
            Nil => true,
            Cons(_, _) => false,
        }
    }

    fn cons(&self, x: Rc<T>) -> Self {
        let t = self.clone();
        Stream(Susp::new(Box::new(|| Cons(x, t))))
    }

    fn head(&self) -> Option<Rc<T>> {
        // returnもsuspension?
        match &*self.0.get() {
            Nil => None,
            Cons(x, _) => Some(x.clone()),
        }
    }

    fn tail(&self) -> Option<Self> {
        // returnもsuspension?
        match &*self.0.get() {
            Nil => None,
            Cons(_, t) => Some(t.clone()),
        }
    }

    fn append(&self, t: &Self) -> Self {
        unimplemented!()
    }

    fn take(&self, n: usize) -> Self {
        unimplemented!()
    }

    fn drop(&self, n: usize) -> Self {
        unimplemented!()
    }

    fn reverse(s: Self) -> Self {
        unimplemented!()
    }
}

mod tests {
    use crate::chap04::stack::Stack;
    use crate::chap04::stream::Stream;
    use std::rc::Rc;

    #[test]
    fn test_stream() {
        let n = Stream::empty();
        let s1 = n.cons(Rc::new(3));
        let s2 = Stream::empty().cons(Rc::new(s1.clone()));

        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        let _ = s2.head();
        println!();
        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        let _ = s1.head();
        println!();
        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        let _ = s1.tail();
        println!();
        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);
    }
}
