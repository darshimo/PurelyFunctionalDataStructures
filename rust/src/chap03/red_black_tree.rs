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
        fn ins<T: Ordered + Clone>(s: &RedBlackTree<T>, x: T) -> RedBlackTree<T> {
            match &*s.0 {
                E => RedBlackTree(Rc::new(T(
                    R,
                    RedBlackTree::empty(),
                    x,
                    RedBlackTree::empty(),
                ))),
                T(color, a, y, b) => {
                    if x.lt(y) {
                        RedBlackTree(Rc::new(T(*color, ins(a, x), y.clone(), b.clone()))).lbalance()
                    } else if y.lt(&x) {
                        RedBlackTree(Rc::new(T(*color, a.clone(), y.clone(), ins(b, x)))).rbalance()
                    } else {
                        s.clone()
                    }
                }
            }
        }

        match &*ins(self, x).0 {
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
}

mod test {
    use crate::common::{ordered::Ordered, set::Set};

    use super::RedBlackTree;

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

    #[test]
    fn test() {
        let set = RedBlackTree::empty()
            .insert(U32(3))
            .insert(U32(5))
            .insert(U32(1))
            .insert(U32(3))
            .insert(U32(4));

        println!("{}", set.member(U32(1)));
        println!("{}", set.member(U32(2)));
    }
}
