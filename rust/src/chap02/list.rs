use std::fmt::Debug;

use crate::common::stack::Stack;

use super::custom_stack::CustomStack;

pub struct List<T>(usize, CustomStack<T>);

impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        List(self.0, self.1.clone())
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.1)
    }
}

impl<T: Clone> FromIterator<T> for List<T> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut it = iter.into_iter();
        match it.next() {
            Some(x) => it.collect::<List<_>>().cons(x),
            None => List::empty(),
        }
    }
}

impl<T: Clone> Stack<T> for List<T> {
    fn empty() -> Self {
        List(0, CustomStack::empty())
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    fn cons(&self, x: T) -> Self {
        List(self.0 + 1, self.1.cons(x))
    }

    fn head(&self) -> Result<T, String> {
        Ok(self.get()?.0)
    }

    fn tail(&self) -> Result<Self, String> {
        Ok(self.get()?.1)
    }
}

impl<T: Clone> List<T> {
    pub fn len(&self) -> usize {
        self.0
    }

    pub fn get(&self) -> Result<(T, List<T>), String> {
        let (x, t) = self.1.get().map_err(|_| "empty list.")?;
        Ok((x, List(self.0 - 1, t)))
    }

    pub fn map<U: Clone, F: Fn(T) -> U>(&self, f: F) -> List<U> {
        List(self.0, self.1.map(f))
    }

    pub fn reverse(&self) -> Self {
        fn reverse_<T: Clone>(l1: &List<T>, l2: &List<T>) -> List<T> {
            match l1.get() {
                Ok((x, t)) => reverse_(&t, &l2.cons(x)),
                Err(_) => l2.clone(),
            }
        }

        reverse_(self, &List::empty())
    }

    pub fn enumerate(&self) -> List<(usize, T)> {
        fn enumerate_<T: Clone>(i: usize, l: &List<T>) -> List<(usize, T)> {
            match l.get() {
                Ok((x, t)) => enumerate_(i + 1, &t).cons((i, x)),
                Err(_) => List::empty(),
            }
        }

        enumerate_(0, self)
    }

    pub fn split(&self, n: usize) -> (List<T>, List<T>) {
        if n == 0 {
            (List::empty(), self.clone())
        } else if let Ok((x, t)) = self.get() {
            let (l1, l2) = t.split(n - 1);
            (l1.cons(x), l2)
        } else {
            (List::empty(), List::empty())
        }
    }
}

mod test {
    use std::io::empty;

    use crate::common::stack::Stack;

    use super::List;

    #[test]
    fn test_from_iterator() {
        let l: List<u32> = (0..5).collect();
        println!("{}", l.len());
        println!("{:?}", l);
    }

    #[test]
    fn test_get() {
        println!("{:?}", List::<u32>::empty().get());
        println!("{:?}", List::empty().cons(0).get());
    }

    #[test]
    fn test_map() {
        let l = (0..10).collect::<List<u32>>().map(|x| x + 100);
        println!("{}", l.len());
        println!("{:?}", l);
    }

    #[test]
    fn test_enumerate() {
        let l = [2, 4, 7, 3, 7, 8, 3, 45, 6]
            .into_iter()
            .collect::<List<u32>>()
            .enumerate();
        println!("{}", l.len());
        println!("{:?}", l);
    }
}
