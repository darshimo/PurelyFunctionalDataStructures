// 図2.9

use std::rc::Rc;

use super::super::common::{ordered::Ordered, set::Set};

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
            E => UnbalancedSet(Rc::new(T(
                UnbalancedSet::empty(),
                x,
                UnbalancedSet::empty(),
            ))),
            T(a, y, b) => {
                if x.lt(y) {
                    UnbalancedSet(Rc::new(T(a.insert(x), y.clone(), b.clone())))
                } else if y.lt(&x) {
                    UnbalancedSet(Rc::new(T(a.clone(), y.clone(), b.insert(x))))
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
