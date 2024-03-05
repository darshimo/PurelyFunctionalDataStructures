use std::rc::Rc;

// 図2.1
pub trait Stack<T>: Sized {
    fn empty() -> Self;
    fn is_empty(&self) -> bool;

    fn cons(&self, x: T) -> Self;
    fn head(&self) -> Rc<T>;
    fn tail(&self) -> Self;
}
