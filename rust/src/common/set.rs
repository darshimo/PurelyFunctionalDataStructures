// 図2.7

pub trait Set<T> {
    fn empty() -> Self;
    fn insert(&self, x: T) -> Self;
    fn member(&self, x: T) -> bool;
}
