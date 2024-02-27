use crate::{lazy, lazy_from};

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
        Stream(lazy!(Nil))
    }

    fn is_empty(&self) -> Susp<bool> {
        let s = self.clone();

        lazy!(match &*s.0.get() {
            Nil => true,
            Cons(_, _) => false,
        })
    }

    fn cons(&self, x: Rc<T>) -> Self {
        let s = self.clone();

        Stream(lazy!(Cons(x, s)))
    }

    fn head(&self) -> Susp<Rc<T>> {
        let s = self.clone();

        lazy!(match &*s.0.get() {
            Nil => panic!("empty stream."),
            Cons(x, _) => x.clone(),
        })
    }

    fn tail(&self) -> Self {
        let s = self.clone();

        Stream(lazy_from!(match &*s.0.get() {
            Nil => panic!("empty stream."),
            Cons(_, t) => t.0.clone(),
        }))
    }

    fn append(&self, t: &Self) -> Self {
        let s = self.clone();
        let t = t.clone();

        Stream(lazy_from!(match &*s.0.get() {
            Nil => t.0,
            Cons(x, s) => {
                let x = x.clone();
                let s = s.clone();
                lazy!(Cons(x, s.append(&t)))
            }
        }))
    }

    fn take(&self, n: usize) -> Self {
        let s = self.clone();

        Stream(lazy_from!(if n > 0 {
            match &*s.0.get() {
                Nil => lazy!(Nil),
                Cons(x, s) => {
                    let x = x.clone();
                    let s = s.clone();
                    lazy!(Cons(x, s.take(n - 1)))
                }
            }
        } else {
            lazy!(Nil)
        }))
    }

    fn drop(&self, n: usize) -> Self {
        fn drop_<T>(s: &Stream<T>, m: usize) -> Stream<T> {
            if m > 0 {
                match &*s.0.get() {
                    Nil => Stream(lazy!(Nil)),
                    Cons(_, s) => {
                        1;
                        drop_(s, m - 1)
                    }
                }
            } else {
                s.clone()
            }
        }

        let s = self.clone();

        Stream(lazy_from!(drop_(&s, n).0))
    }

    fn reverse(&self) -> Self {
        fn reverse_<T: 'static>(s: &Stream<T>, r: Stream<T>) -> Stream<T> {
            match &*s.0.get() {
                Nil => r,
                Cons(x, s) => reverse_(s, r.cons(x.clone())),
            }
        }

        let s = self.clone();

        Stream(lazy_from!(reverse_(&s, Stream::empty()).0))
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

        println!();
        let x = &*s2.get().head().get();
        if let Some(x) = x {
            println!("{}", x);
        }
        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        println!();
        let _ = s1.get().head();
        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        println!();
        let _ = s1.get().tail();
        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        println!();
        let n = Stream::empty();
        let s = n.get().cons(Rc::new(1));
        let s = s.get().cons(Rc::new(2));
        let s = s.get().cons(Rc::new(3));
        let s = s.get().cons(Rc::new(4));
        let s = s.get().cons(Rc::new(5));
        let t = (&*s.get()).drop(2);
        println!("{}", s);
        println!("{:?}", t.get().head().get());
        println!("{}", s);
        println!("{}", t);
    }
}
