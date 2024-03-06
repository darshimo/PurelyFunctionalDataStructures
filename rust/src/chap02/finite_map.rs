// 図2.10

pub trait FiniteMap<K, V> {
    fn empty() -> Self;
    fn bind(&self, key: K, value: V) -> Self;
    fn lookup(&self, key: K) -> V;
}
