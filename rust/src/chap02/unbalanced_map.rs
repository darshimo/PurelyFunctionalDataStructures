// 演習問題 2.6

use std::rc::Rc;

use super::super::common::{finite_map::FiniteMap, ordered::Ordered};

struct UnbalancedMap<K, V>(Rc<UnbalancedMapCell<K, V>>);

enum UnbalancedMapCell<K, V> {
    E,
    T(UnbalancedMap<K, V>, K, V, UnbalancedMap<K, V>),
}
use UnbalancedMapCell::*;

impl<K, V> Clone for UnbalancedMap<K, V> {
    fn clone(&self) -> Self {
        UnbalancedMap(self.0.clone())
    }
}

impl<K: Ordered + Clone, V: Clone> FiniteMap<K, V> for UnbalancedMap<K, V> {
    fn empty() -> Self {
        UnbalancedMap(Rc::new(E))
    }

    fn bind(&self, key: K, value: V) -> Self {
        match &*self.0 {
            E => UnbalancedMap(Rc::new(T(
                UnbalancedMap::empty(),
                key,
                value,
                UnbalancedMap::empty(),
            ))),
            T(a, key_, value_, b) => {
                if key.lt(key_) {
                    UnbalancedMap(Rc::new(T(
                        a.bind(key, value),
                        key_.clone(),
                        value_.clone(),
                        b.clone(),
                    )))
                } else if key_.lt(&key) {
                    UnbalancedMap(Rc::new(T(
                        a.clone(),
                        key_.clone(),
                        value_.clone(),
                        b.bind(key, value),
                    )))
                } else {
                    UnbalancedMap(Rc::new(T(a.clone(), key, value, b.clone())))
                }
            }
        }
    }

    fn lookup(&self, key: K) -> Result<V, String> {
        match &*self.0 {
            E => Err("not found.".to_string()),
            T(a, key_, value_, b) => {
                if key.lt(key_) {
                    a.lookup(key)
                } else if key_.lt(&key) {
                    b.lookup(key)
                } else {
                    Ok(value_.clone())
                }
            }
        }
    }
}
