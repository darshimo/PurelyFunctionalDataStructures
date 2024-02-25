use super::suspension::Susp;
use std::rc::Rc;

// 図2.1
pub trait Stack<T>: Sized {
    fn empty() -> Susp<Self>;
    fn is_empty(&self) -> Susp<bool>;

    fn cons(&self, x: Rc<T>) -> Susp<Self>;
    fn head(&self) -> Susp<Option<Rc<T>>>;
    fn tail(&self) -> Susp<Option<Self>>;

    fn append(&self, t: &Self) -> Susp<Self>;
    fn take(&self, n: usize) -> Susp<Self>;
    fn drop(&self, n: usize) -> Susp<Self>;
    fn reverse(s: Self) -> Susp<Self>;
}
