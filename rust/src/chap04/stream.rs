// 図4.1

use std::fmt::{Debug, Display};

use crate::{common::suspension::Susp, lazy, lazy_from};

struct Stream<T>(Susp<StreamCell<T>>);

enum StreamCell<T> {
    Nil,
    Cons(T, Stream<T>),
}
use StreamCell::*;

impl<T: Clone> Clone for StreamCell<T> {
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

impl<T: Debug> Debug for StreamCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Nil => write!(f, "[]"),
            Cons(x, t) => write!(f, "({:?}) :: {:?}", x, t),
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

impl<T: Debug> Debug for Stream<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<T: 'static + Clone> crate::common::stream::Stream<T> for Stream<T> {
    fn empty() -> Self {
        Stream(lazy!(Nil))
    }

    fn is_empty(&self) -> Susp<bool> {
        let s = self.clone();

        lazy!(match s.0.get() {
            Nil => true,
            Cons(_, _) => false,
        })
    }

    fn cons(&self, x: T) -> Self {
        let s = self.clone();

        Stream(lazy!(Cons(x, s)))
    }

    fn head(&self) -> Susp<T> {
        let s = self.clone();

        lazy!(match s.0.get() {
            Nil => panic!("empty stream."),
            Cons(x, _) => x.clone(),
        })
    }

    fn tail(&self) -> Self {
        let s = self.clone();

        Stream(lazy_from!(match s.0.get() {
            Nil => panic!("empty stream."),
            Cons(_, t) => t.0.clone(),
        }))
    }

    fn append(&self, t: &Self) -> Self {
        let s = self.clone();
        let t = t.clone();

        Stream(lazy_from!(match s.0.get() {
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
            match s.0.get() {
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
        fn drop_<T: Clone>(s: Stream<T>, m: usize) -> Stream<T> {
            if m > 0 {
                match s.0.get() {
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

        Stream(lazy_from!(drop_(s, n).0))
    }

    fn reverse(&self) -> Self {
        fn reverse_<T: 'static + Clone>(s: Stream<T>, r: Stream<T>) -> Stream<T> {
            match s.0.get() {
                Nil => r,
                Cons(x, s) => reverse_(s, r.cons(x.clone())),
            }
        }

        let s = self.clone();

        Stream(lazy_from!(reverse_(s, Stream::empty()).0))
    }
}

mod tests {

    use std::rc::Rc;

    use crate::common::stream::Stream;

    #[test]
    fn test_stream() {
        let n = super::Stream::<u32>::empty();
        let s1 = n.cons(3);
        let s2 = super::Stream::empty().cons(s1.clone());

        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        println!();
        let x = s2.head().get();
        println!("{}", x);
        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        println!();
        let _ = s1.head().get();
        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        println!();
        let _ = s1.tail().is_empty().get();
        println!("n : {}", n);
        println!("s1: {}", s1);
        println!("s2: {}", s2);

        println!();
        let n = super::Stream::<u32>::empty();
        let s = n.cons(5);
        let s = s.cons(4);
        let s = s.cons(3);
        let s = s.cons(2);
        let s = s.cons(1);
        let t = s.drop(2);
        println!("{}", s);
        println!("{:?}", t.head().get());
        println!("{}", s);
        println!("{}", t);
    }

    #[derive(Debug)]
    struct U32(u32);
    impl Clone for U32 {
        fn clone(&self) -> Self {
            println!("clone: {}!", self.0);
            let mut _i = 0u64;
            for _ in 0..50000000u64 {
                _i += 1;
            }
            U32(self.0)
        }
    }

    #[test]
    fn test_stream_heavy() {
        let n = super::Stream::<U32>::empty();
        let s = n.cons(U32(3));
        let s = s.cons(U32(2));
        let s = s.cons(U32(1));
        let s = s.take(3);

        println!("s : {:?}", s);
        let _ = s.tail().tail().tail().is_empty().get();
        println!("s : {:?}", s);
    }

    #[test]
    fn test_stream_heavy_rc() {
        let n = super::Stream::<Rc<U32>>::empty();
        let s = n.cons(Rc::new(U32(3)));
        let s = s.cons(Rc::new(U32(2)));
        let s = s.cons(Rc::new(U32(1)));
        let s = s.take(3);

        println!("s : {:?}", s);
        let _ = s.tail().tail().tail().is_empty().get();
        println!("s : {:?}", s);
    }
}
