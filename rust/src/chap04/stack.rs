use super::super::common::suspension::Susp;

// 図2.1
pub trait Stack<T>: Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> Susp<bool>;

    fn cons(&self, x: T) -> Self;
    fn head(&self) -> Susp<T>;
    fn tail(&self) -> Self;

    fn append(&self, t: &Self) -> Self;
    fn take(&self, n: usize) -> Self;
    fn drop(&self, n: usize) -> Self;
    fn reverse(&self) -> Self;
}
