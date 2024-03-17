// 演習問題 5.8(b)

use std::rc::Rc;

use crate::common::{heap::Heap, ordered::Ordered};

#[derive(Clone)]
pub struct PairingHeap<T>(Rc<PairingHeapCell<T>>);

enum PairingHeapCell<T> {
    E_,
    T_(PairingHeap<T>, T, PairingHeap<T>),
}
use PairingHeapCell::*;

impl<T: Ordered + Clone> Heap<T> for PairingHeap<T> {
    fn empty() -> Self {
        PairingHeap(Rc::new(E_))
    }

    fn is_empty(&self) -> bool {
        match &*self.0 {
            E_ => true,
            T_(_, _, _) => false,
        }
    }

    fn insert(&self, x: T) -> Self {
        self.merge(&PairingHeap(Rc::new(T_(
            PairingHeap::empty(),
            x,
            PairingHeap::empty(),
        ))))
    }

    fn merge(&self, other: &Self) -> Self {
        match (&*self.0, &*other.0) {
            (_, E_) => self.clone(),
            (E_, _) => other.clone(),
            (T_(a, x, _), T_(b, y, _)) => {
                if x.leq(y) {
                    PairingHeap(Rc::new(T_(
                        PairingHeap(Rc::new(T_(b.clone(), y.clone(), a.clone()))),
                        x.clone(),
                        PairingHeap::empty(),
                    )))
                } else {
                    PairingHeap(Rc::new(T_(
                        PairingHeap(Rc::new(T_(a.clone(), x.clone(), b.clone()))),
                        y.clone(),
                        PairingHeap::empty(),
                    )))
                }
            }
        }
    }

    fn find_min(&self) -> Result<T, String> {
        match &*self.0 {
            E_ => Err("empty heap.".to_string()),
            T_(_, x, _) => Ok(x.clone()),
        }
    }

    fn delete_min(&self) -> Result<Self, String> {
        fn merge_pairs<T: Ordered + Clone>(h: &PairingHeap<T>) -> PairingHeap<T> {
            match &*h.0 {
                T_(a, x, b) => {
                    let h1 = PairingHeap::from(a.clone(), x.clone(), PairingHeap::empty());
                    match &*b.0 {
                        T_(c, y, d) => {
                            let h2 = PairingHeap::from(c.clone(), y.clone(), PairingHeap::empty());
                            h1.merge(&h2).merge(&merge_pairs(d))
                        }
                        E_ => h1,
                    }
                }
                E_ => PairingHeap::empty(),
            }
        }

        match &*self.0 {
            E_ => Err("empty heap.".to_string()),
            T_(h, _, _) => Ok(merge_pairs(h)),
        }
    }
}

impl<T> PairingHeap<T> {
    pub fn from(left: Self, x: T, right: Self) -> Self {
        PairingHeap(Rc::new(T_(left, x, right)))
    }
}
