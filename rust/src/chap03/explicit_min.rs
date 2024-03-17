// 演習問題 3.7

use crate::common::{heap::Heap, ordered::Ordered};

#[derive(Clone)]
struct ExplicitMin<T, H: Heap<T>>(ExplicitMinCell<T, H>);
#[derive(Clone)]
enum ExplicitMinCell<T, H> {
    E,
    NE(T, H),
}
use ExplicitMinCell::*;

impl<T: Ordered + Clone, H: Heap<T> + Clone> Heap<T> for ExplicitMin<T, H> {
    fn empty() -> Self {
        ExplicitMin(E)
    }

    fn is_empty(&self) -> bool {
        match self.0 {
            E => true,
            NE(_, _) => false,
        }
    }

    fn insert(&self, x: T) -> Self {
        match &self.0 {
            E => ExplicitMin(NE(x.clone(), H::empty().insert(x))),
            NE(y, h) => ExplicitMin(NE(
                if x.leq(y) { x.clone() } else { y.clone() },
                h.insert(x),
            )),
        }
    }

    fn merge(&self, other: &Self) -> Self {
        match (&self.0, &other.0) {
            (_, E) => self.clone(),
            (E, _) => other.clone(),
            (NE(x, h1), NE(y, h2)) => ExplicitMin(NE(
                if x.leq(y) { x.clone() } else { y.clone() },
                h1.merge(h2),
            )),
        }
    }

    fn find_min(&self) -> Result<T, String> {
        match &self.0 {
            E => Err("empty heap.".to_string()),
            NE(t, _) => Ok(t.clone()),
        }
    }

    fn delete_min(&self) -> Result<Self, String> {
        match &self.0 {
            E => Err("empty heap.".to_string()),
            NE(_, ih) => match ih.delete_min() {
                Ok(ih_) => match ih_.find_min() {
                    Ok(x) => Ok(ExplicitMin(NE(x, ih_))),
                    Err(_) => Ok(ExplicitMin::empty()),
                },
                Err(_) => unreachable!("ih is supposed to have some elements."),
            },
        }
    }
}

mod test {
    use crate::{
        chap03::{binomial_heap_new::BinomialHeap, explicit_min::ExplicitMin},
        common::{heap::Heap, int::Int},
    };

    #[test]
    fn test() {
        let mut h: ExplicitMin<_, BinomialHeap<_>> = ExplicitMin::empty();
        for n in [1, 6, 4, 4, 8, 3, 6, 7, 9, 5, 4, 2, 3, 6, 4, 8, 4] {
            h = h.insert(Int(n));
        }

        let mut v = vec![];
        while !h.is_empty() {
            v.push(h.find_min().unwrap().0);
            h = h.delete_min().unwrap();
        }
        println!("{:?}", v);
    }

    #[test]
    fn test_merge() {
        let h1 = {
            let mut h: ExplicitMin<_, BinomialHeap<_>> = ExplicitMin::empty();
            for n in [1, 6, 4, 4, 8, 3, 6, 7, 9, 5, 4, 2, 3, 6, 4, 8, 4] {
                h = h.insert(Int(n));
            }
            h
        };

        let h2 = {
            let mut h: ExplicitMin<_, BinomialHeap<_>> = ExplicitMin::empty();
            for n in [7, 2, 5, 6, 8, 4, 2, 7, 3, 3, 2, 7, 2, 7, 3, 9, 8, 3, 5] {
                h = h.insert(Int(n));
            }
            h
        };

        let mut h = h1.merge(&h2);

        let mut v = vec![];
        while !h.is_empty() {
            v.push(h.find_min().unwrap().0);
            h = h.delete_min().unwrap();
        }
        println!("{:?}", v);
    }
}
