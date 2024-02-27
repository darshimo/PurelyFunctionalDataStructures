use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

#[macro_export]
macro_rules! lazy {
    ( $f : expr ) => {
        Susp::new(Box::new(move || $f))
    };
}

#[macro_export]
macro_rules! lazy_from {
    ( $f : expr ) => {
        Susp::from(Box::new(move || $f))
    };
}

pub struct Susp<T>(Rc<RefCell<Inner<T>>>);

enum Inner<T> {
    Fun(Box<dyn FnOnce() -> T>),
    Val(Rc<T>),
    Tmp(Box<dyn FnOnce() -> Susp<T>>),
}
use Inner::*;

impl<T> Susp<T> {
    pub fn new(f: Box<dyn FnOnce() -> T>) -> Self {
        Susp(Rc::new(RefCell::new(Fun(f))))
    }

    pub fn get(&self) -> Rc<T> {
        let ret = match self.0.replace(Fun(Box::new(|| unreachable!()))) {
            Fun(f) => Rc::new(f()),
            Val(x) => x,
            Tmp(f) => f().get(),
        };

        *self.0.borrow_mut() = Val(ret.clone());

        ret
    }

    pub fn from(f: Box<dyn FnOnce() -> Susp<T>>) -> Self {
        Susp(Rc::new(RefCell::new(Tmp(f))))
    }
}

impl<T> Clone for Susp<T> {
    fn clone(&self) -> Self {
        Susp(self.0.clone())
    }
}

impl<T: Debug> Debug for Susp<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self.0.borrow() {
            Val(x) => {
                write!(f, "{:?}", x)
            }
            _ => {
                write!(f, "<LazyFun>")
            }
        }
    }
}

impl<T: Display> Display for Susp<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self.0.borrow() {
            Val(x) => {
                write!(f, "{}", x)
            }
            _ => {
                write!(f, "<LazyFun>")
            }
        }
    }
}

mod tests {
    use super::Susp;

    fn heavy_func(s: &str) -> u32 {
        println!("{}", s);
        let mut ret = 0;
        for _ in 0..300000000 {
            ret += 1;
        }
        ret
    }

    #[test]
    fn test_suspension() {
        // 評価前のSuspensionをcloneしてから評価しても，評価結果が共有されることを確認
        let s1 = lazy!(heavy_func("[calc01]"));
        println!("clone");
        let s2 = s1.clone();

        let tmp1 = s1.clone();
        let tmp2 = s2.clone();
        let s3 = lazy!(if true { tmp1 } else { tmp2 });

        println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
        println!();

        println!("get1");
        let _ = s1.get(); // 遅い
        println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
        println!();

        println!("get2");
        let _ = s2.get(); // 速い
        println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
        println!();

        println!("get3");
        let _ = s3.get(); // 速い
        println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
        println!();
    }

    #[test]
    fn test_suspension2() {
        // 評価前のSuspensionをcloneしてから評価しても，評価結果が共有されることを確認
        let s1 = Susp::new(Box::new(|| heavy_func("[calc01]")));
        println!("clone");
        let s2 = s1.clone();

        let tmp1 = s1.clone();
        let tmp2 = s2.clone();
        let s3 = Susp::from(Box::new(move || if true { tmp1 } else { tmp2 }));

        println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
        println!();

        println!("get3");
        let _ = s3.get(); // 遅い
        println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
        println!();

        println!("get1");
        let _ = s1.get(); // 速い
        println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
        println!();

        println!("get2");
        let _ = s2.get(); // 速い
        println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
        println!();
    }
}
