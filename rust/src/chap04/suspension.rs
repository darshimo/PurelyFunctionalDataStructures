use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

#[derive(Clone)]
pub struct Suspension<T>(Rc<RefCell<Inner<T>>>);

enum Inner<T> {
    Fun(Box<dyn FnOnce() -> T>),
    Val(Rc<T>),
}
use Inner::*;

impl<T> Suspension<T> {
    pub fn new(f: Box<dyn FnOnce() -> T>) -> Self {
        Suspension(Rc::new(RefCell::new(Fun(f))))
    }

    pub fn get(&self) -> Rc<T> {
        let ret;
        match &mut *self.0.borrow_mut() {
            Fun(r) => {
                let f = std::mem::replace(r, Box::new(|| panic!("hogefuga")));
                ret = Rc::new(f())
            }
            Val(x) => ret = x.clone(),
        }

        *self.0.borrow_mut() = Val(ret.clone());
        ret
    }
}

impl<T: Debug> Debug for Suspension<T> {
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

impl<T: Display> Display for Suspension<T> {
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
    use super::Suspension;

    fn heavy_func() -> u32 {
        let mut ret = 0;
        for _ in 0..300000000 {
            ret += 1;
        }
        ret
    }

    #[test]
    fn test_suspension() {
        // 評価前のSuspensionをcloneしてから評価しても，評価結果が共有されることを確認
        let s1 = Suspension::new(Box::new(|| heavy_func()));
        println!("clone");
        let s2 = s1.clone();
        println!("get1");
        let _ = s1.get(); // 遅い
        println!("get2");
        let _ = s2.get(); // 速い
        println!("fin");
    }
}
