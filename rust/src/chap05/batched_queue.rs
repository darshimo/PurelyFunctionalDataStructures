// 図5.2

use crate::{
    chap02::list::List,
    common::{queue::Queue, stack::Stack},
};

#[derive(Clone)]
struct BatchedQueue<T>(List<T>, List<T>);

impl<T: Clone> Queue<T> for BatchedQueue<T> {
    fn empty() -> Self {
        BatchedQueue(List::empty(), List::empty())
    }

    fn is_empty(&self) -> bool {
        let BatchedQueue(f, _) = self;

        f.is_empty()
    }

    fn snoc(&self, x: T) -> Self {
        let BatchedQueue(f, r) = self;

        BatchedQueue(f.clone(), r.cons(x)).checkf()
    }

    fn head(&self) -> T {
        let BatchedQueue(f, _) = self;

        f.head().map_err(|_| "empty queue.").unwrap()
    }

    fn tail(&self) -> Self {
        let BatchedQueue(f, r) = self;

        let f_ = f.tail().map_err(|_| "empty queue.").unwrap();

        BatchedQueue(f_, r.clone()).checkf()
    }
}

impl<T: Clone> BatchedQueue<T> {
    fn checkf(&self) -> Self {
        let BatchedQueue(f, r) = self;

        if f.is_empty() {
            BatchedQueue(r.reverse(), List::empty())
        } else {
            self.clone()
        }
    }
}

mod test {
    use crate::common::queue::Queue;

    use super::BatchedQueue;

    #[test]
    fn test() {
        let q_0 = BatchedQueue::empty().snoc(0);
        let q_01 = q_0.snoc(1);
        println!("head of q_01: {}", q_01.head());
        let q_1 = q_01.tail();
        let q_12 = q_1.snoc(2);
        println!("head of q_12: {}", q_12.head());
        let q_2 = q_12.tail();
        println!("head of q_2: {}", q_2.head());
        let q_ = q_2.tail();
        println!("is q_ empty?: {}", q_.is_empty());
    }
}
