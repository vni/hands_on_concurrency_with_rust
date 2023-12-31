// #[cfg(test)]
// extern crate quickcheck;

mod hash_map_u8;
pub use hash_map_u8::HashMapU8;

use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
use std::{cmp, mem /* , ptr */};

#[derive(Default)]
pub struct HashMap<K, V, S = RandomState>
where
    K: Eq,
    V: ::std::fmt::Debug,
{
    hash_builder: S,
    data: Vec<(u64, K, V)>,
}

impl<K: Eq, V> HashMap<K, V, RandomState>
where
    K: Eq + Hash,
    V: ::std::fmt::Debug,
{
    pub fn new() -> HashMap<K, V> {
        HashMap {
            hash_builder: RandomState::new(),
            data: Vec::new(),
        }
    }
}

fn make_hash<T: ?Sized, S>(hash_builder: &S, t: &T) -> u64
where
    T: Hash,
    S: BuildHasher,
{
    let mut state = hash_builder.build_hasher();
    t.hash(&mut state);
    state.finish()
}

impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
    V: ::std::fmt::Debug,
{
    pub fn with_hasher(hash_builder: S) -> HashMap<K, V, S> {
        HashMap {
            hash_builder: hash_builder,
            data: Vec::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let hash = make_hash(&self.hash_builder, &k);

        let end = self.data.len();
        for idx in 0..end {
            match self.data[idx].0.cmp(&hash) {
                cmp::Ordering::Greater => {
                    self.data.insert(idx, (hash, k, v));
                    return None;
                }
                cmp::Ordering::Less => continue,
                cmp::Ordering::Equal => {
                    let old = mem::replace(&mut self.data[idx].2, v);
                    return Some(old);
                }
            }
        }
        self.data.push((hash, k, v));
        None
    }

    pub fn get<Q: ?Sized>(&mut self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q> + ::std::fmt::Debug,
        Q: Hash + Eq + ::std::fmt::Debug,
    {
        let hash = make_hash(&self.hash_builder, k);

        for &(bucket_hash, _, ref v) in &self.data {
            if hash == bucket_hash {
                return Some(v);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};

    #[test]
    fn get_what_you_give() {
        fn property(k: u16, v: u16) -> TestResult {
            let mut system_under_test = HashMap::new(); // SUT

            assert_eq!(None, system_under_test.insert(k, v));
            assert_eq!(Some(&v), system_under_test.get(&k));

            TestResult::passed()
        }
        QuickCheck::new().quickcheck(property as fn(u16, u16) -> TestResult);
    }

    #[derive(Clone, Debug)]
    enum Action<T>
    where
        T: Arbitrary,
    {
        Insert(T, u16),
        Lookup(T),
    }

    impl<T> Arbitrary for Action<T>
    where
        T: Arbitrary,
    {
        fn arbitrary<G>(g: &mut G) -> Action<T>
        where
            G: Gen,
        {
            let i: usize = g.gen_range(0, 100);
            match i {
                0..=50 => Action::Insert(Arbitrary::arbitrary(g), u16::arbitrary(g)),
                _ => Action::Lookup(Arbitrary::arbitrary(g)),
            }
        }
    }

    #[test]
    fn sut_vs_genuine_article() {
        fn property2<T>(actions: Vec<Action<T>>) -> TestResult
        where
            T: Arbitrary + Eq + Hash + ::std::fmt::Debug,
        {
            let mut model = ::std::collections::HashMap::new();
            let mut system_under_test = HashMap::new();

            for action in actions.into_iter() {
                match action {
                    Action::Insert(k, v) => {
                        assert_eq!(model.insert(k.clone(), v), system_under_test.insert(k, v));
                    }
                    Action::Lookup(k) => {
                        assert_eq!(model.get(&k), system_under_test.get(&k));
                    }
                }
            }
            TestResult::passed()
        }
        QuickCheck::new().quickcheck(property2 as fn(Vec<Action<u8>>) -> TestResult);
    }
}
