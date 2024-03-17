// 図5.6

// struct PairingHeap<T>()

use crate::{
    chap02::list::List,
    common::{heap::Heap, ordered::Ordered, stack::Stack},
};

#[derive(Clone)]
enum PairingHeap<T> {
    E,
    T(T, List<PairingHeap<T>>),
}
use PairingHeap::*;

impl<T: Ordered + Clone> Heap<T> for PairingHeap<T> {
    fn empty() -> Self {
        E
    }

    fn is_empty(&self) -> bool {
        match self {
            E => true,
            T(_, _) => false,
        }
    }

    fn insert(&self, x: T) -> Self {
        self.merge(&T(x, List::empty()))
    }

    fn merge(&self, other: &Self) -> Self {
        match (self, other) {
            (_, E) => self.clone(),
            (E, _) => other.clone(),
            (T(x, l1), T(y, l2)) => {
                if x.leq(y) {
                    T(x.clone(), l1.cons(other.clone()))
                } else {
                    T(y.clone(), l2.cons(self.clone()))
                }
            }
        }
    }

    fn find_min(&self) -> Result<T, String> {
        match self {
            E => Err("empty heap.".to_string()),
            T(x, _) => Ok(x.clone()),
        }
    }

    fn delete_min(&self) -> Result<Self, String> {
        fn merge_pairs<T: Ordered + Clone>(l: &List<PairingHeap<T>>) -> PairingHeap<T> {
            match l.get() {
                Ok((x, l1)) => match l1.get() {
                    Ok((y, l2)) => x.merge(&y).merge(&merge_pairs(&l2)),
                    Err(_) => x,
                },
                Err(_) => E,
            }
        }

        match self {
            E => Err("empty heap.".to_string()),
            T(_, l) => Ok(merge_pairs(l)),
        }
    }
}
