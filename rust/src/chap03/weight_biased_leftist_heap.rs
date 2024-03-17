// 演習問題 3.4

use std::rc::Rc;

use crate::{
    chap02::list::List,
    common::{heap::Heap, ordered::Ordered, stack::Stack},
};

struct WeightBiasedLeftistHeap<T>(Rc<WeightBiasedLeftistHeapCell<T>>);

enum WeightBiasedLeftistHeapCell<T> {
    E,
    T(
        u32,
        T,
        WeightBiasedLeftistHeap<T>,
        WeightBiasedLeftistHeap<T>,
    ),
}
use WeightBiasedLeftistHeapCell::*;

impl<T> Clone for WeightBiasedLeftistHeap<T> {
    fn clone(&self) -> Self {
        WeightBiasedLeftistHeap(self.0.clone())
    }
}

impl<T: Ordered + Clone> Heap<T> for WeightBiasedLeftistHeap<T> {
    fn empty() -> Self {
        WeightBiasedLeftistHeap(Rc::new(E))
    }

    fn is_empty(&self) -> bool {
        match &*self.0 {
            E => true,
            T(_, _, _, _) => false,
        }
    }

    fn insert(&self, x: T) -> Self {
        self.merge(&WeightBiasedLeftistHeap(Rc::new(T(
            1,
            x,
            Self::empty(),
            Self::empty(),
        ))))
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

impl<T: Ordered + Clone> FromIterator<T> for WeightBiasedLeftistHeap<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        WeightBiasedLeftistHeap::from_list(iter.into_iter().collect())
    }
}

impl<T: Ordered + Clone> WeightBiasedLeftistHeap<T> {
    fn make_t(x: &T, a: &Self, b: &Self) -> Self {
        if a.rank() >= b.rank() {
            WeightBiasedLeftistHeap(Rc::new(T(
                a.rank() + b.rank() + 1,
                x.clone(),
                a.clone(),
                b.clone(),
            )))
        } else {
            WeightBiasedLeftistHeap(Rc::new(T(
                a.rank() + b.rank() + 1,
                x.clone(),
                b.clone(),
                a.clone(),
            )))
        }
    }

    fn rank(&self) -> u32 {
        match &*self.0 {
            E => 0,
            T(r, _, _, _) => *r,
        }
    }

    fn insert_without_merge(&self, x: T) -> Self {
        match &*self.0 {
            E => WeightBiasedLeftistHeap(Rc::new(T(1, x, Self::empty(), Self::empty()))),
            T(_, y, a, b) => {
                if x.leq(y) {
                    Self::make_t(&x, a, &b.insert_without_merge(y.clone()))
                } else {
                    Self::make_t(y, a, &b.insert_without_merge(x))
                }
            }
        }
    }

    fn from_list(l: List<T>) -> Self {
        let mut heap_list: List<_> = l.map(|x| WeightBiasedLeftistHeap::empty().insert(x));

        while heap_list.len() > 1 {
            heap_list = Self::list_merge(heap_list);
        }

        if let Ok(h) = heap_list.head() {
            h
        } else {
            Self::empty()
        }
    }

    fn list_merge(l: List<WeightBiasedLeftistHeap<T>>) -> List<WeightBiasedLeftistHeap<T>> {
        match l.get() {
            Ok((h1, l1)) => match l1.get() {
                Ok((h2, l2)) => Self::list_merge(l2).cons(h1.merge(&h2)),
                Err(_) => l,
            },
            Err(_) => l,
        }
    }

    fn merge_top_down(&self, other: &Self) -> Self {
        match (&*self.0, &*other.0) {
            (_, E) => self.clone(),
            (E, _) => other.clone(),
            (T(r1, x, a1, b1), T(r2, y, a2, b2)) => {
                if x.leq(y) {
                    if a1.rank() >= b1.rank() + other.rank() {
                        WeightBiasedLeftistHeap(Rc::new(T(
                            r1 + r2,
                            x.clone(),
                            a1.clone(),
                            b1.merge_top_down(other),
                        )))
                    } else {
                        WeightBiasedLeftistHeap(Rc::new(T(
                            r1 + r2,
                            x.clone(),
                            b1.merge_top_down(other),
                            a1.clone(),
                        )))
                    }
                } else {
                    if a2.rank() >= self.rank() + b2.rank() {
                        WeightBiasedLeftistHeap(Rc::new(T(
                            r1 + r2,
                            y.clone(),
                            a2.clone(),
                            self.merge_top_down(b2),
                        )))
                    } else {
                        WeightBiasedLeftistHeap(Rc::new(T(
                            r1 + r2,
                            y.clone(),
                            self.merge_top_down(b2),
                            a2.clone(),
                        )))
                    }
                }
            }
        }
    }
}

mod test {
    use crate::{
        chap03::weight_biased_leftist_heap::WeightBiasedLeftistHeap,
        common::{heap::Heap, int::Int},
    };

    #[test]
    fn test_from_list() {
        let mut h = WeightBiasedLeftistHeap::from_list(
            [1, 6, 4, 5, 8, 3, 1, 9, 9, 7, 2, 0]
                .into_iter()
                .map(|n| Int(n))
                .collect(),
        );

        let mut v = vec![];
        while !h.is_empty() {
            v.push(h.find_min().unwrap().0);
            h = h.delete_min().unwrap();
        }
        println!("{:?}", v);
    }

    #[test]
    fn test_from_iter() {
        let mut h: WeightBiasedLeftistHeap<_> = [1, 6, 4, 5, 8, 3, 1, 9, 9, 7, 2, 0]
            .into_iter()
            .map(|n| Int(n))
            .collect();

        let mut v = vec![];
        while !h.is_empty() {
            v.push(h.find_min().unwrap().0);
            h = h.delete_min().unwrap();
        }
        println!("{:?}", v);
    }
}
