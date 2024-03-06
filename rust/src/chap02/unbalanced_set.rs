// 図2.9

use std::rc::Rc;

use crate::common::{ordered::Ordered, set::Set};

struct UnbalancedSet<T>(Rc<UnbalancedSetCell<T>>);
enum UnbalancedSetCell<T> {
    E,
    T(UnbalancedSet<T>, T, UnbalancedSet<T>),
}
use UnbalancedSetCell::*;

impl<T> Clone for UnbalancedSet<T> {
    fn clone(&self) -> Self {
        UnbalancedSet(self.0.clone())
    }
}

impl<T: Ordered + Clone> Set<T> for UnbalancedSet<T> {
    fn empty() -> Self {
        UnbalancedSet(Rc::new(E))
    }

    fn insert(&self, x: T) -> Self {
        match &*self.0 {
            E => UnbalancedSet::from(UnbalancedSet::empty(), x, UnbalancedSet::empty()),
            T(a, y, b) => {
                if x.lt(y) {
                    UnbalancedSet::from(a.insert(x), y.clone(), b.clone())
                } else if y.lt(&x) {
                    UnbalancedSet::from(a.clone(), y.clone(), b.insert(x))
                } else {
                    self.clone()
                }
            }
        }
    }

    fn member(&self, x: T) -> bool {
        match &*self.0 {
            E => false,
            T(a, y, b) => {
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

impl<T: Ordered + Clone> UnbalancedSet<T> {
    fn from(l: Self, x: T, r: Self) -> Self {
        UnbalancedSet(Rc::new(T(l, x, r)))
    }

    // 演習問題 2.2
    fn member_fast(&self, x: T) -> bool {
        fn inner<U: Ordered>(t: &UnbalancedSet<U>, x: U, candidate: Option<&U>) -> bool {
            match (&*t.0, candidate) {
                (E, Some(y)) => x.eq(y),
                (E, None) => false,
                (T(a, y, b), _) => {
                    if x.lt(y) {
                        inner(a, x, candidate)
                    } else {
                        inner(b, x, Some(y))
                    }
                }
            }
        }

        inner(self, x, None)
    }

    // 演習問題 2.3
    fn insert_only_unique(&self, x: T) -> Result<Self, String> {
        match &*self.0 {
            E => Ok(UnbalancedSet::from(
                UnbalancedSet::empty(),
                x,
                UnbalancedSet::empty(),
            )),
            T(a, y, b) => {
                if x.lt(y) {
                    let left = a.insert_only_unique(x)?;
                    Ok(UnbalancedSet::from(left, y.clone(), b.clone()))
                } else if y.lt(&x) {
                    let right = b.insert_only_unique(x)?;
                    Ok(UnbalancedSet::from(a.clone(), y.clone(), right))
                } else {
                    Err("already exists.".to_string())
                }
            }
        }
    }

    // 演習問題 2.4
    fn insert_fast_only_unique(&self, x: T) -> Result<Self, String> {
        fn inner<U: Ordered + Clone>(
            t: &UnbalancedSet<U>,
            x: U,
            candidate: Option<&U>,
        ) -> Result<UnbalancedSet<U>, String> {
            match (&*t.0, candidate) {
                (E, Some(y)) => {
                    if x.eq(y) {
                        Err("already exists.".to_string())
                    } else {
                        Ok(UnbalancedSet::from(
                            UnbalancedSet::empty(),
                            x,
                            UnbalancedSet::empty(),
                        ))
                    }
                }
                (E, None) => Ok(UnbalancedSet::from(
                    UnbalancedSet::empty(),
                    x,
                    UnbalancedSet::empty(),
                )),
                (T(a, y, b), _) => {
                    if x.lt(y) {
                        let left = inner(a, x, candidate)?;
                        Ok(UnbalancedSet::from(left, y.clone(), b.clone()))
                    } else {
                        let right = inner(b, x, Some(y))?;
                        Ok(UnbalancedSet::from(a.clone(), y.clone(), right))
                    }
                }
            }
        }

        inner(self, x, None)
    }

    // 演習問題 2.5(a)
    fn complete_from_depth(x: T, d: usize) -> Self {
        if d == 0 {
            UnbalancedSet::empty()
        } else {
            let t = UnbalancedSet::complete_from_depth(x.clone(), d - 1);
            UnbalancedSet::from(t.clone(), x, t)
        }
    }

    // 演習問題 2.5(b)
    fn complete_from_size(x: T, n: usize) -> Self {
        let mut vec = vec![None; n + 1];

        fn inner<T: Ordered + Clone>(
            x: T,
            n: usize,
            vec: &mut Vec<Option<UnbalancedSet<T>>>,
        ) -> UnbalancedSet<T> {
            if let Some(s) = vec[n].clone() {
                s
            } else if n == 0 {
                vec[n] = Some(UnbalancedSet::empty());
                UnbalancedSet::empty()
            } else {
                let nl = n / 2;
                let nr = n - 1 - nl;
                let l = UnbalancedSet::complete_from_size(x.clone(), nl);
                let r = UnbalancedSet::complete_from_size(x.clone(), nr);
                let s = UnbalancedSet::from(l, x, r);
                vec[n] = Some(s.clone());
                s
            }
        }

        inner(x, n, &mut vec)
    }
}
