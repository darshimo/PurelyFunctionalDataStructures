use crate::{
    chap02::list::List,
    common::{deque::Deque, stack::Stack},
};

#[derive(Clone, Debug)]
struct BatchedDeque<T>(List<T>, List<T>);

impl<T: Clone> Deque<T> for BatchedDeque<T> {
    fn empty() -> Self {
        BatchedDeque(List::empty(), List::empty())
    }

    fn is_empty(&self) -> bool {
        let BatchedDeque(f, _) = self;

        f.is_empty()
    }

    fn cons(&self, x: T) -> Self {
        let BatchedDeque(f, r) = self;

        BatchedDeque(f.cons(x), r.clone()).check()
    }

    fn head(&self) -> T {
        let BatchedDeque(f, r) = self;

        f.head().or(r.head()).map_err(|_| "empty deque.").unwrap()
    }

    fn tail(&self) -> Self {
        let BatchedDeque(f, r) = self;

        match f.tail() {
            Ok(f_) => BatchedDeque(f_, r.clone()).check(),
            Err(_) => {
                if r.is_empty() {
                    panic!("empty deque.")
                } else {
                    BatchedDeque::empty()
                }
            }
        }
    }

    fn snoc(&self, x: T) -> Self {
        let BatchedDeque(f, r) = self;

        BatchedDeque(f.clone(), r.cons(x)).check()
    }

    fn last(&self) -> T {
        let BatchedDeque(f, r) = self;

        r.head().or(f.head()).map_err(|_| "empty deque.").unwrap()
    }

    fn init(&self) -> Self {
        let BatchedDeque(f, r) = self;

        match r.tail() {
            Ok(r_) => BatchedDeque(f.clone(), r_).check(),
            Err(_) => {
                if f.is_empty() {
                    panic!("empty deque.")
                } else {
                    BatchedDeque::empty()
                }
            }
        }
    }
}

impl<T: Clone> BatchedDeque<T> {
    fn check(&self) -> Self {
        fn devide<T: Clone>(l: &List<T>) -> (List<T>, List<T>) {
            let n1 = (l.len() + 1) / 2;
            let (l1, l2) = l.split(n1);
            (l1, l2.reverse())
        }

        let BatchedDeque(f, r) = self;

        match (f.is_empty(), r.is_empty()) {
            (true, false) => {
                let (l1, l2) = devide(r);
                BatchedDeque(l2, l1)
            }
            (false, true) => {
                let (l1, l2) = devide(f);
                BatchedDeque(l1, l2)
            }
            _ => self.clone(),
        }
    }
}

mod test {
    use crate::common::deque::Deque;

    use super::BatchedDeque;

    #[test]
    fn test() {
        {
            let dq = BatchedDeque::empty().cons(0);
            println!("0\t: {:?}", dq);

            let dq = dq.cons(1);
            println!("10\t: {:?}", dq);

            let dq = dq.snoc(2);
            println!("102\t: {:?}", dq);

            let dq = dq.snoc(3);
            println!("1023\t: {:?}", dq);

            let dq = dq.snoc(4);
            println!("10234\t: {:?}", dq);

            let dq = dq.snoc(5);
            println!("102345\t: {:?}", dq);

            let dq = dq.tail();
            println!("02345\t: {:?}", dq);
        }

        println!();

        {
            let dq = BatchedDeque::empty().snoc(0);
            println!("0\t: {:?}", dq);

            let dq = dq.snoc(1);
            println!("01\t: {:?}", dq);

            let dq = dq.cons(2);
            println!("201\t: {:?}", dq);

            let dq = dq.cons(3);
            println!("3201\t: {:?}", dq);

            let dq = dq.cons(4);
            println!("43201\t: {:?}", dq);

            let dq = dq.cons(5);
            println!("543201\t: {:?}", dq);

            let dq = dq.init();
            println!("54320\t: {:?}", dq);
        }
    }
}
