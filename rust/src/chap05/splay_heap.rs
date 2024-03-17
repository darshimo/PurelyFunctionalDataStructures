// 図5.5
use std::rc::Rc;

use crate::{
    chap02::list::List,
    common::{heap::Heap, ordered::Ordered, stack::Stack},
};

#[derive(Clone)]
struct SplayHeap<T>(Rc<SplayHeapCell<T>>);
enum SplayHeapCell<T> {
    E,
    T(SplayHeap<T>, T, SplayHeap<T>),
}
use SplayHeapCell::*;

macro_rules! sh {
    ( $l : expr, $x : expr, $r : expr ) => {
        SplayHeap(Rc::new(T($l, $x, $r)))
    };
}

impl<T: Ordered + Clone> Heap<T> for SplayHeap<T> {
    fn empty() -> Self {
        SplayHeap(Rc::new(E))
    }

    fn is_empty(&self) -> bool {
        match &*self.0 {
            E => true,
            T(_, _, _) => false,
        }
    }

    fn insert(&self, x: T) -> Self {
        let (l, r) = self.partition(x.clone());
        sh!(l, x, r)
    }

    fn merge(&self, other: &Self) -> Self {
        match &*self.0 {
            E => other.clone(),
            T(a, x, b) => {
                let (l, r) = other.partition(x.clone());
                sh!(l.merge(a), x.clone(), r.merge(b))
            }
        }
    }

    fn find_min(&self) -> Result<T, String> {
        match &*self.0 {
            E => Err("empty heap.".to_string()),
            T(a, x, b) => {
                if a.is_empty() {
                    Ok(x.clone())
                } else {
                    a.find_min()
                }
            }
        }
    }

    fn delete_min(&self) -> Result<Self, String> {
        match &*self.0 {
            E => Err("empty heap.".to_string()),
            T(a, x, b) => match &*a.0 {
                E => Ok(b.clone()),
                T(c, y, d) => match c.delete_min() {
                    Ok(c_) => Ok(sh!(c_, y.clone(), sh!(d.clone(), x.clone(), b.clone()))),
                    Err(_) => Ok(sh!(d.clone(), x.clone(), b.clone())),
                },
            },
        }
    }
}

impl<T: Ordered + Clone> SplayHeap<T> {
    // 演習問題 5.4
    fn smaller(&self, pivot: T) -> Self {
        match &*self.0 {
            E => SplayHeap::empty(),
            T(a, x, b) => {
                if x.leq(&pivot) {
                    match &*b.0 {
                        E => self.clone(),
                        T(c, y, d) => {
                            if y.leq(&pivot) {
                                sh!(
                                    sh!(a.clone(), x.clone(), c.clone()),
                                    y.clone(),
                                    d.smaller(pivot)
                                )
                            } else {
                                sh!(a.clone(), x.clone(), c.smaller(pivot))
                            }
                        }
                    }
                } else {
                    a.smaller(pivot)
                }
            }
        }
    }

    fn bigger(&self, pivot: T) -> Self {
        match &*self.0 {
            E => SplayHeap::empty(),
            T(a, x, b) => {
                if x.leq(&pivot) {
                    b.bigger(pivot)
                } else {
                    match &*a.0 {
                        E => self.clone(),
                        T(c, y, d) => {
                            if y.leq(&pivot) {
                                sh!(d.bigger(pivot), x.clone(), b.clone())
                            } else {
                                sh!(
                                    c.bigger(pivot),
                                    y.clone(),
                                    sh!(d.clone(), x.clone(), b.clone())
                                )
                            }
                        }
                    }
                }
            }
        }
    }

    fn partition(&self, pivot: T) -> (Self, Self) {
        match &*self.0 {
            E => (SplayHeap::empty(), SplayHeap::empty()),
            T(a, x, b) => {
                if x.leq(&pivot) {
                    match &*b.0 {
                        E => (self.clone(), SplayHeap::empty()),
                        T(c, y, d) => {
                            if y.leq(&pivot) {
                                let (l, r) = d.partition(pivot);
                                (sh!(sh!(a.clone(), x.clone(), c.clone()), y.clone(), l), r)
                            } else {
                                let (l, r) = c.partition(pivot);
                                (sh!(a.clone(), x.clone(), l), sh!(r, y.clone(), d.clone()))
                            }
                        }
                    }
                } else {
                    match &*a.0 {
                        E => (SplayHeap::empty(), self.clone()),
                        T(c, y, d) => {
                            if y.leq(&pivot) {
                                let (l, r) = d.partition(pivot);
                                (sh!(c.clone(), y.clone(), l), sh!(r, x.clone(), b.clone()))
                            } else {
                                let (l, r) = c.partition(pivot);
                                (l, sh!(r, y.clone(), sh!(d.clone(), x.clone(), b.clone())))
                            }
                        }
                    }
                }
            }
        }
    }
}
