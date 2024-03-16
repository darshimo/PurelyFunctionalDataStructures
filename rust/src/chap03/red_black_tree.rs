use std::rc::Rc;

use crate::{
    chap02::list::List,
    common::{ordered::Ordered, set::Set},
};

#[derive(Clone, Copy)]
enum Color {
    R,
    B,
}
use Color::*;

#[derive(Clone)]
struct RedBlackTree<T>(Rc<RedBlackTreeCell<T>>);
enum RedBlackTreeCell<T> {
    E,
    T(Color, RedBlackTree<T>, T, RedBlackTree<T>),
}
use RedBlackTreeCell::*;

impl<T: Ordered + Clone> Set<T> for RedBlackTree<T> {
    fn empty() -> Self {
        RedBlackTree(Rc::new(E))
    }

    fn insert(&self, x: T) -> Self {
        enum Route {
            Left,
            Right,
            Neither,
        }
        use Route::*;

        fn ins<T: Ordered + Clone>(s: &RedBlackTree<T>, x: T) -> (RedBlackTree<T>, Route) {
            match &*s.0 {
                E => (
                    RedBlackTree(Rc::new(T(
                        R,
                        RedBlackTree::empty(),
                        x,
                        RedBlackTree::empty(),
                    ))),
                    Neither,
                ),
                T(color, a, y, b) => {
                    if x.lt(y) {
                        let (a_with_x, route) = ins(a, x);

                        let s_with_x =
                            RedBlackTree(Rc::new(T(*color, a_with_x, y.clone(), b.clone())));

                        let s_with_x_balanced = match route {
                            Left => s_with_x.llbalance(),
                            Right => s_with_x.lrbalance(),
                            Neither => s_with_x,
                        };

                        (s_with_x_balanced, Left)
                    } else if y.lt(&x) {
                        let (b_with_x, route) = ins(b, x);

                        let s_with_x =
                            RedBlackTree(Rc::new(T(*color, a.clone(), y.clone(), b_with_x)));

                        let s_with_x_balanced = match route {
                            Left => s_with_x.rlbalance(),
                            Right => s_with_x.rrbalance(),
                            Neither => s_with_x,
                        };

                        (s_with_x_balanced, Right)
                    } else {
                        (s.clone(), Neither)
                    }
                }
            }
        }

        match &*ins(self, x).0 .0 {
            T(_, a, y, b) => RedBlackTree(Rc::new(T(B, a.clone(), y.clone(), b.clone()))),
            E => unreachable!(),
        }
    }

    fn member(&self, x: T) -> bool {
        match &*self.0 {
            E => false,
            T(_, a, y, b) => {
                if x.lt(y) {
                    a.member(x)
                } else if y.lt(&x) {
                    b.member(x)
                } else {
                    true
                }
            }
        }
    }
}

impl<T: Ordered + Clone> RedBlackTree<T> {
    fn balance(&self) -> Self {
        if let T(B, a, x, b) = &*self.0 {
            if let T(R, c, y, d) = &*a.0 {
                if let T(R, e, z, f) = &*c.0 {
                    let l = RedBlackTree(Rc::new(T(B, e.clone(), z.clone(), f.clone())));
                    let r = RedBlackTree(Rc::new(T(B, d.clone(), x.clone(), b.clone())));
                    let t = RedBlackTree(Rc::new(T(R, l, y.clone(), r)));
                    return t;
                }

                if let T(R, e, z, f) = &*d.0 {
                    let l = RedBlackTree(Rc::new(T(B, c.clone(), y.clone(), e.clone())));
                    let r = RedBlackTree(Rc::new(T(B, f.clone(), x.clone(), b.clone())));
                    let t = RedBlackTree(Rc::new(T(R, l, z.clone(), r)));
                    return t;
                }
            }

            if let T(R, c, y, d) = &*b.0 {
                if let T(R, e, z, f) = &*c.0 {
                    let l = RedBlackTree(Rc::new(T(B, a.clone(), x.clone(), e.clone())));
                    let r = RedBlackTree(Rc::new(T(B, f.clone(), y.clone(), d.clone())));
                    let t = RedBlackTree(Rc::new(T(R, l, z.clone(), r)));
                    return t;
                }

                if let T(R, e, z, f) = &*d.0 {
                    let l = RedBlackTree(Rc::new(T(B, a.clone(), x.clone(), c.clone())));
                    let r = RedBlackTree(Rc::new(T(B, e.clone(), z.clone(), f.clone())));
                    let t = RedBlackTree(Rc::new(T(R, l, y.clone(), r)));
                    return t;
                }
            }
        }

        self.clone()
    }

    // 演習問題 3.9
    fn from_ord_list(l: List<T>) -> Self {
        fn inner<T: Ordered + Clone>(
            l: List<T>,
            n: usize,
            d: usize,
            m: usize,
        ) -> (RedBlackTree<T>, List<T>) {
            if n == 0 {
                return (RedBlackTree::empty(), l);
            }

            let left_size = (n - 1) / 2;
            let right_size = (n - 1) - left_size;

            let (left, l) = inner(l, left_size, d + 1, m);
            let (x, l) = l.get().unwrap();
            let (right, l) = inner(l, right_size, d + 1, m);

            (
                RedBlackTree(Rc::new(T(if d < m { B } else { R }, left, x, right))),
                l,
            )
        }

        let n = l.len();
        let m = (n as f32).log2() as usize;

        inner(l, n, 0, m).0
    }

    // 演習問題 3.10(a)
    fn lbalance(&self) -> Self {
        if let T(B, a, x, b) = &*self.0 {
            if let T(R, c, y, d) = &*a.0 {
                if let T(R, e, z, f) = &*c.0 {
                    let l = RedBlackTree(Rc::new(T(B, e.clone(), z.clone(), f.clone())));
                    let r = RedBlackTree(Rc::new(T(B, d.clone(), x.clone(), b.clone())));
                    let t = RedBlackTree(Rc::new(T(R, l, y.clone(), r)));
                    return t;
                }

                if let T(R, e, z, f) = &*d.0 {
                    let l = RedBlackTree(Rc::new(T(B, c.clone(), y.clone(), e.clone())));
                    let r = RedBlackTree(Rc::new(T(B, f.clone(), x.clone(), b.clone())));
                    let t = RedBlackTree(Rc::new(T(R, l, z.clone(), r)));
                    return t;
                }
            }
        }

        self.clone()
    }

    fn rbalance(&self) -> Self {
        if let T(B, a, x, b) = &*self.0 {
            if let T(R, c, y, d) = &*b.0 {
                if let T(R, e, z, f) = &*c.0 {
                    let l = RedBlackTree(Rc::new(T(B, a.clone(), x.clone(), e.clone())));
                    let r = RedBlackTree(Rc::new(T(B, f.clone(), y.clone(), d.clone())));
                    let t = RedBlackTree(Rc::new(T(R, l, z.clone(), r)));
                    return t;
                }

                if let T(R, e, z, f) = &*d.0 {
                    let l = RedBlackTree(Rc::new(T(B, a.clone(), x.clone(), c.clone())));
                    let r = RedBlackTree(Rc::new(T(B, e.clone(), z.clone(), f.clone())));
                    let t = RedBlackTree(Rc::new(T(R, l, y.clone(), r)));
                    return t;
                }
            }
        }

        self.clone()
    }

    // 演習問題 3.10(b)
    fn llbalance(&self) -> Self {
        if let T(B, a, x, b) = &*self.0 {
            if let T(R, c, y, d) = &*a.0 {
                if let T(R, e, z, f) = &*c.0 {
                    let l = RedBlackTree(Rc::new(T(B, e.clone(), z.clone(), f.clone())));
                    let r = RedBlackTree(Rc::new(T(B, d.clone(), x.clone(), b.clone())));
                    let t = RedBlackTree(Rc::new(T(R, l, y.clone(), r)));
                    return t;
                }
            }
        }

        self.clone()
    }

    fn lrbalance(&self) -> Self {
        if let T(B, a, x, b) = &*self.0 {
            if let T(R, c, y, d) = &*a.0 {
                if let T(R, e, z, f) = &*d.0 {
                    let l = RedBlackTree(Rc::new(T(B, c.clone(), y.clone(), e.clone())));
                    let r = RedBlackTree(Rc::new(T(B, f.clone(), x.clone(), b.clone())));
                    let t = RedBlackTree(Rc::new(T(R, l, z.clone(), r)));
                    return t;
                }
            }
        }

        self.clone()
    }

    fn rlbalance(&self) -> Self {
        if let T(B, a, x, b) = &*self.0 {
            if let T(R, c, y, d) = &*b.0 {
                if let T(R, e, z, f) = &*c.0 {
                    let l = RedBlackTree(Rc::new(T(B, a.clone(), x.clone(), e.clone())));
                    let r = RedBlackTree(Rc::new(T(B, f.clone(), y.clone(), d.clone())));
                    let t = RedBlackTree(Rc::new(T(R, l, z.clone(), r)));
                    return t;
                }
            }
        }

        self.clone()
    }

    fn rrbalance(&self) -> Self {
        if let T(B, a, x, b) = &*self.0 {
            if let T(R, c, y, d) = &*b.0 {
                if let T(R, e, z, f) = &*d.0 {
                    let l = RedBlackTree(Rc::new(T(B, a.clone(), x.clone(), c.clone())));
                    let r = RedBlackTree(Rc::new(T(B, e.clone(), z.clone(), f.clone())));
                    let t = RedBlackTree(Rc::new(T(R, l, y.clone(), r)));
                    return t;
                }
            }
        }

        self.clone()
    }
}

mod test {
    use crate::{
        chap02::list::List,
        common::{ordered::Ordered, set::Set, stack::Stack},
    };

    use super::{Color, Color::*, RedBlackTree, RedBlackTreeCell::*};

    #[derive(Clone)]
    struct U32(u32);
    impl Ordered for U32 {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }

        fn leq(&self, other: &Self) -> bool {
            self.0 <= other.0
        }

        fn lt(&self, other: &Self) -> bool {
            self.0 < other.0
        }
    }

    impl<T: Ordered + Clone> RedBlackTree<T> {
        fn is_balanced(&self) -> bool {
            fn check<T>(s: &RedBlackTree<T>) -> Result<(Color, usize), ()> {
                match &*s.0 {
                    E => Ok((B, 1)),
                    T(color, a, _, b) => {
                        let (c1, n1) = check(a)?;
                        let (c2, n2) = check(b)?;

                        if let (R, R, _) | (R, _, R) = (color, c1, c2) {
                            return Err(());
                        }

                        if n1 != n2 {
                            return Err(());
                        }

                        Ok((*color, if let B = color { n1 + 1 } else { n1 }))
                    }
                }
            }

            match check(self) {
                Ok(_) => true,
                Err(_) => false,
            }
        }

        fn traverse(&self) -> List<T> {
            fn inner<T: Clone>(s: &RedBlackTree<T>, l: List<T>) -> List<T> {
                match &*s.0 {
                    E => l,
                    T(_, a, x, b) => {
                        let l = inner(b, l);
                        let l = l.cons(x.clone());
                        let l = inner(a, l);
                        l
                    }
                }
            }

            inner(self, List::empty())
        }
    }

    #[test]
    fn test() {
        let set = RedBlackTree::empty()
            .insert(U32(3))
            .insert(U32(5))
            .insert(U32(1))
            .insert(U32(3))
            .insert(U32(2))
            .insert(U32(7))
            .insert(U32(5))
            .insert(U32(8))
            .insert(U32(9))
            .insert(U32(3))
            .insert(U32(6))
            .insert(U32(10))
            .insert(U32(3))
            .insert(U32(4));

        println!("1 is member?: {}", set.member(U32(1)));
        println!("2 is member?: {}", set.member(U32(2)));
        println!("is balanced? : {}", set.is_balanced());
        println!("{:?}", set.traverse().map(|x| x.0));
    }

    #[test]
    fn test_from_ord_list() {
        let l = List::empty()
            .cons(U32(12))
            .cons(U32(11))
            .cons(U32(10))
            .cons(U32(9))
            .cons(U32(8))
            .cons(U32(7))
            .cons(U32(6))
            .cons(U32(5))
            .cons(U32(4))
            .cons(U32(3))
            .cons(U32(2))
            .cons(U32(1))
            .cons(U32(0));

        let set = RedBlackTree::from_ord_list(l);

        println!("12 is member?: {}", set.member(U32(12)));
        println!("13 is member?: {}", set.member(U32(13)));
        println!("is balanced? : {}", set.is_balanced());
        println!("{:?}", set.traverse().map(|x| x.0));
    }
}
