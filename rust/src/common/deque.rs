pub trait Deque<T> {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;

    fn cons(&self, x: T) -> Self;
    fn head(&self) -> T;
    fn tail(&self) -> Self;

    fn snoc(&self, x: T) -> Self;
    fn last(&self) -> T;
    fn init(&self) -> Self;
}
