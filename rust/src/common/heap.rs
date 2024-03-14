// 図3.1

pub trait Heap<T>: Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;

    fn insert(&self, x: T) -> Self;
    fn merge(&self, other: &Self) -> Self;

    fn find_min(&self) -> Result<T, String>;
    fn delete_min(&self) -> Result<Self, String>;
}
