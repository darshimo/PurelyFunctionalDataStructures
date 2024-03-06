// 図2.9

pub trait Ordered {
    fn eq(&self, other: &Self) -> bool;
    fn lt(&self, other: &Self) -> bool;
    fn leq(&self, other: &Self) -> bool;
}
