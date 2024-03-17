use std::fmt::{Debug, Display};

use super::ordered::Ordered;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Int(pub i32);

impl Ordered for Int {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn lt(&self, other: &Self) -> bool {
        self.0 < other.0
    }

    fn leq(&self, other: &Self) -> bool {
        self.0 <= other.0
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Debug for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
