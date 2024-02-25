use super::{stack::Stack, suspension::Susp};
use std::{fmt::Display, rc::Rc};

struct Stream<T>(Susp<StreamCell<T>>);

enum StreamCell<T> {
    Nil,
    Cons(Rc<T>, Susp<Stream<T>>),
}
use StreamCell::*;

impl<T> Clone for StreamCell<T> {
    fn clone(&self) -> Self {
        match self {
            Nil => Nil,
            Cons(x, t) => Cons(x.clone(), t.clone()),
        }
    }
}

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
    fn empty() -> Susp<Self> {
        Susp::new(Box::new(|| Stream(Susp::new(Box::new(|| Nil)))))
    }

    fn is_empty(&self) -> Susp<bool> {
        let s = self.clone();

        Susp::new(Box::new(move || match &*s.0.get() {
            Nil => true,
            Cons(_, _) => false,
        }))
    }

    fn cons(&self, x: Rc<T>) -> Susp<Self> {
        let s = self.clone();
        Susp::new(Box::new(|| {
            Stream(Susp::new(Box::new(|| Cons(x, Susp::new(Box::new(|| s))))))
        }))
    }

    fn head(&self) -> Susp<Option<Rc<T>>> {
        let s = self.clone();
        Susp::new(Box::new(move || match &*s.0.get() {
            Nil => None,
            Cons(x, _) => Some(x.clone()),
        }))
    }

    fn tail(&self) -> Susp<Option<Self>> {
        let s = self.clone();
        Susp::new(Box::new(move || match &*s.0.get() {
            Nil => None,
            Cons(_, t) => Some((&*t.get()).clone()),
        }))
    }

    fn append(&self, t: &Self) -> Susp<Self> {
        unimplemented!()
    }

    fn take(&self, n: usize) -> Susp<Self> {
        unimplemented!()
    }

    fn drop(&self, n: usize) -> Susp<Self> {
        unimplemented!()
    }

    fn reverse(s: Self) -> Susp<Self> {
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
        let s1 = n.get().cons(Rc::new(3));
        let s2 = Stream::empty().get().cons(Rc::new(s1.clone()));

        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        let _ = s2.get().head();
        println!();
        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        let _ = s1.get().head();
        println!();
        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        let _ = s1.get().tail();
        println!();
        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);
    }
}
