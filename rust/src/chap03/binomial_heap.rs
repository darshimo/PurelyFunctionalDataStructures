use crate::{
    chap02::list::List,
    common::{heap::Heap, ordered::Ordered, stack::Stack},
};

#[derive(Clone)]
struct BinomialHeap<T>(List<Tree<T>>);

#[derive(Clone)]
struct Tree<T>(usize, T, List<Tree<T>>);

impl<T: Ordered + Clone> Heap<T> for BinomialHeap<T> {
    fn empty() -> Self {
        BinomialHeap(List::empty())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn insert(&self, x: T) -> Self {
        self.ins_tree(Tree(0, x, List::empty()))
    }

    fn merge(&self, other: &Self) -> Self {
        match (self.0.get(), other.0.get()) {
            (_, Err(_)) => self.clone(),
            (Err(_), _) => other.clone(),
            (Ok((t1, ts1)), Ok((t2, ts2))) => {
                if t1.rank() < t2.rank() {
                    BinomialHeap(BinomialHeap(ts1).merge(other).0.cons(t1))
                } else if t2.rank() < t1.rank() {
                    BinomialHeap(self.merge(&BinomialHeap(ts2)).0.cons(t2))
                } else {
                    BinomialHeap(ts1)
                        .merge(&BinomialHeap(ts2))
                        .ins_tree(t1.link(&t2))
                }
            }
        }
    }

    fn find_min(&self) -> Result<T, String> {
        Ok(self.remove_min_tree()?.0.root())
    }

    fn delete_min(&self) -> Result<Self, String> {
        let (Tree(_, _, ts1), ts2) = self.remove_min_tree()?;
        Ok(BinomialHeap(ts1.reverse()).merge(&ts2))
    }
}

impl<T: Ordered + Clone> BinomialHeap<T> {
    fn ins_tree(&self, t: Tree<T>) -> Self {
        if let Ok((t_, ts_)) = self.0.get() {
            if t.rank() < t_.rank() {
                BinomialHeap(self.0.cons(t))
            } else {
                BinomialHeap(ts_).ins_tree(t.link(&t_))
            }
        } else {
            BinomialHeap(List::empty().cons(t))
        }
    }

    fn remove_min_tree(&self) -> Result<(Tree<T>, Self), String> {
        let (t, ts) = self.0.get().map_err(|_| "empty heap.")?;

        match BinomialHeap(ts.clone()).remove_min_tree() {
            Ok((t_, ts_)) => {
                if t.root().leq(&t_.root()) {
                    Ok((t, BinomialHeap(ts)))
                } else {
                    Ok((t_, BinomialHeap(ts_.0.cons(t))))
                }
            }
            Err(_) => Ok((t, BinomialHeap::empty())),
        }
    }
}

impl<T: Ordered + Clone> Tree<T> {
    fn link(&self, other: &Self) -> Self {
        let Tree(r, x1, c1) = self;
        let Tree(_, x2, c2) = other;

        if x1.leq(x2) {
            Tree(r + 1, x1.clone(), c1.cons(other.clone()))
        } else {
            Tree(r + 1, x2.clone(), c2.cons(self.clone()))
        }
    }

    fn rank(&self) -> usize {
        self.0
    }

    fn root(&self) -> T {
        self.1.clone()
    }
}

mod test {
    use crate::common::{heap::Heap, ordered::Ordered};

    use super::BinomialHeap;

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
        let mut h = BinomialHeap::empty();
        for n in [1, 6, 4, 4, 8, 3, 6, 7, 9, 5, 4, 2, 3, 6, 4, 8, 4] {
            h = h.insert(U32(n));
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
            let mut h = BinomialHeap::empty();
            for n in [1, 6, 4, 4, 8, 3, 6, 7, 9, 5, 4, 2, 3, 6, 4, 8, 4] {
                h = h.insert(U32(n));
            }
            h
        };

        let h2 = {
            let mut h = BinomialHeap::empty();
            for n in [7, 2, 5, 6, 8, 4, 2, 7, 3, 3, 2, 7, 2, 7, 3, 9, 8, 3, 5] {
                h = h.insert(U32(n));
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
