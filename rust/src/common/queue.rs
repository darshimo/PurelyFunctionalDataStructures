// 図5.1

pub trait Queue<T> {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;

    fn snoc(&self, x: T) -> Self;
    fn head(&self) -> T;
    fn tail(&self) -> Self;
}
