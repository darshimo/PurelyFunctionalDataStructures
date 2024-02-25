use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

pub struct Susp<T>(Rc<RefCell<Inner<T>>>);

enum Inner<T> {
    Fun(Box<dyn FnOnce() -> T>),
    Val(T),
}
use Inner::*;

impl<T> Susp<T> {
    pub fn new(f: Box<dyn FnOnce() -> T>) -> Self {
        Susp(Rc::new(RefCell::new(Fun(f))))
    }

    pub fn eval(&self) {
        let v = match &mut *self.0.borrow_mut() {
            Fun(r) => {
                let f = std::mem::replace(r, Box::new(|| panic!("hogefuga")));
                f()
            }
            Val(_) => {
                return;
            }
        };
        *self.0.borrow_mut() = Val(v);
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
            Fun(_) => {
                write!(f, "LazyFun")
            }
            Val(x) => {
                write!(f, "{:?}", x)
            }
        }
    }
}

impl<T: Display> Display for Susp<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self.0.borrow() {
            Fun(_) => {
                write!(f, "LazyFun")
            }
            Val(x) => {
                write!(f, "{}", x)
            }
        }
    }
}

mod tests {
    use super::Susp;

    fn heavy_func() -> u32 {
        let mut ret = 0;
        for _ in 0..500000000 {
            ret += 1;
        }
        ret
    }

    #[test]
    fn it_works() {
        let s = Susp::new(Box::new(heavy_func));

        println!("{}", s);
        s.eval();
        println!("{}", s);
        s.eval();
        println!("{}", s);
    }

    #[test]
    fn test_suspension() {
        // 評価前のSuspensionをcloneしてから評価しても，評価結果が共有されることを確認
        let s1 = Susp::new(Box::new(heavy_func));
        println!("clone");
        let s2 = s1.clone();
        println!("before eval");
        s1.eval(); // 遅い
        println!("v: {}", s1);
        s2.eval(); // 速い
        println!("v: {}", s2);
    }
}
