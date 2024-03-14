// 図3.2

use std::rc::Rc;

use crate::common::{heap::Heap, ordered::Ordered};

struct LeftistHeap<T>(Rc<LeftistHeapCell<T>>);

enum LeftistHeapCell<T> {
    E,
    T(u32, T, LeftistHeap<T>, LeftistHeap<T>),
}
use LeftistHeapCell::*;

impl<T> Clone for LeftistHeap<T> {
    fn clone(&self) -> Self {
        LeftistHeap(self.0.clone())
    }
}

impl<T: Ordered + Clone> Heap<T> for LeftistHeap<T> {
    fn empty() -> Self {
        LeftistHeap(Rc::new(E))
    }

    fn is_empty(&self) -> bool {
        match &*self.0 {
            E => true,
            T(_, _, _, _) => false,
        }
    }

    fn insert(&self, x: T) -> Self {
        self.merge(&Self::from(x))
    }

    fn merge(&self, other: &Self) -> Self {
        match (&*self.0, &*other.0) {
            (_, E) => self.clone(),
            (E, _) => other.clone(),
            (T(_, x, a1, b1), T(_, y, a2, b2)) => {
                if x.leq(y) {
                    Self::make_t(x, a1, &b1.merge(other))
                } else {
                    Self::make_t(y, a2, &self.merge(b2))
                }
            }
        }
    }

    fn find_min(&self) -> Result<T, String> {
        match &*self.0 {
            E => Err("empty heap.".to_string()),
            T(_, x, _, _) => Ok(x.clone()),
        }
    }

    fn delete_min(&self) -> Result<Self, String> {
        match &*self.0 {
            E => Err("empty heap.".to_string()),
            T(_, _, a, b) => Ok(a.merge(b)),
        }
    }
}

impl<T: Ordered + Clone> FromIterator<T> for LeftistHeap<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut heap_vec: Vec<LeftistHeap<T>> =
            iter.into_iter().map(|x| LeftistHeap::from(x)).collect();
        while heap_vec.len() > 1 {
            heap_vec = Self::list_merge(heap_vec);
        }

        if let Some(h) = heap_vec.pop() {
            h
        } else {
            Self::empty()
        }
    }
}

impl<T: Ordered + Clone> LeftistHeap<T> {
    fn make_t(x: &T, a: &Self, b: &Self) -> Self {
        if a.rank() >= b.rank() {
            LeftistHeap(Rc::new(T(b.rank() + 1, x.clone(), a.clone(), b.clone())))
        } else {
            LeftistHeap(Rc::new(T(a.rank() + 1, x.clone(), b.clone(), a.clone())))
        }
    }

    fn rank(&self) -> u32 {
        match &*self.0 {
            E => 0,
            T(r, _, _, _) => *r,
        }
    }

    fn from(x: T) -> Self {
        LeftistHeap(Rc::new(T(1, x, Self::empty(), Self::empty())))
    }

    // 演習問題 3.2
    fn insert_without_merge(&self, x: T) -> Self {
        match &*self.0 {
            E => Self::from(x),
            T(_, y, a, b) => {
                if x.leq(y) {
                    Self::make_t(&x, a, &b.insert_without_merge(y.clone()))
                } else {
                    Self::make_t(y, a, &b.insert_without_merge(x))
                }
            }
        }
    }

    // 演習問題 3.3
    fn from_list(v: Vec<T>) -> Self {
        v.into_iter().collect()
    }

    fn list_merge(mut v: Vec<LeftistHeap<T>>) -> Vec<LeftistHeap<T>> {
        match (v.pop(), v.pop()) {
            (Some(h1), Some(h2)) => {
                let mut ret = Self::list_merge(v);
                ret.push(h1.merge(&h2));
                ret
            }
            (Some(h1), None) => {
                let mut ret = Self::list_merge(v);
                ret.push(h1);
                ret
            }
            _ => vec![],
        }
    }
}

mod test {
    use crate::common::{heap::Heap, ordered::Ordered};

    use super::LeftistHeap;

    #[derive(Clone, Debug)]
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
    fn f() {
        let mut h = LeftistHeap::from_list(
            [1, 6, 4, 5, 8, 3, 1, 9, 9, 7, 2, 0]
                .into_iter()
                .map(|n| U32(n))
                .collect(),
        );

        let mut v = vec![];
        while !h.is_empty() {
            v.push(h.find_min().unwrap().0);
            h = h.delete_min().unwrap();
        }
        println!("{:?}", v);
    }
}
