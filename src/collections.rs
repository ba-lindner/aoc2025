use std::{
    collections::HashMap,
    hash::Hash,
    iter::Sum,
    ops::{Add, AddAssign, MulAssign},
};

use crate::FromVec;

pub trait HashMapExt<K, V> {
    /// Set a value, or update it if the key already exists
    ///
    /// If `k` is not already present in `self`, it is inserted
    /// with the value `v`. Otherwise, `f` is called on a mutable
    /// reference to the currently present value and the given value.
    fn update(&mut self, k: K, v: V, f: impl FnOnce(&mut V, V));

    /// [update](HashMapExt::update()) with an implicit add operation
    fn update_add(&mut self, k: K, v: V)
    where
        V: AddAssign,
    {
        self.update(k, v, |old, new| *old += new);
    }

    /// [update](HashMapExt::update()) with an implicit multiply operation
    fn update_mul(&mut self, k: K, v: V)
    where
        V: MulAssign,
    {
        self.update(k, v, |old, new| *old *= new);
    }

    /// Sum of all values
    fn valsum(&self) -> V
    where
        V: for<'a> Sum<&'a V>;
}

impl<K: Eq + Hash, V> HashMapExt<K, V> for HashMap<K, V> {
    fn update(&mut self, k: K, v: V, f: impl FnOnce(&mut V, V)) {
        if let Some(entry) = self.get_mut(&k) {
            f(entry, v);
        } else {
            self.insert(k, v);
        }
    }

    fn valsum(&self) -> V
    where
        V: for<'a> Sum<&'a V>,
    {
        self.values().sum()
    }
}

/// Extension trait for nested Collections
///
/// For [`HashMap`]s that map to other `HashMap`s or [`Vec`]s,
/// this provides an additional variant of the [`update`](HashMapExt::update) function.
pub trait HashMapExtInsert<K, T> {
    /// [Update](HashMapExt::update) the outer collection
    /// by adding a new inner or appending to it.
    fn update_insert(&mut self, k: K, v: T);
}

impl<K: Eq + Hash, T> HashMapExtInsert<K, T> for HashMap<K, Vec<T>> {
    fn update_insert(&mut self, k: K, v: T) {
        self.update(k, vec![v], |old, mut new| old.append(&mut new));
    }
}

impl<K: Eq + Hash, K1: Eq + Hash, V> HashMapExtInsert<K, (K1, V)> for HashMap<K, HashMap<K1, V>> {
    fn update_insert(&mut self, k: K, v: (K1, V)) {
        self.update(k, HashMap::from([v]), |old, new| {
            for (k, v) in new {
                old.insert(k, v);
            }
        });
    }
}

pub trait SliceExt<T> {
    fn map_ref<U>(&self, f: impl FnMut(&T) -> U) -> Vec<U>;
    fn count(&self, f: impl FnMut(&T) -> bool) -> usize;
    fn add<U>(&self, rhs: &[U]) -> Vec<<T as Add<U>>::Output>
    where
        T: Add<U> + Clone,
        U: Clone;
}

pub trait VecExt<T> {
    fn to_map<V>(self, v: V) -> HashMap<T, V>
    where
        T: Eq + Hash,
        V: Clone;
    fn push_new(&mut self, el: T)
    where
        T: PartialEq;
    fn map<U>(self, f: impl FnMut(T) -> U) -> Vec<U>;
    fn sum(self) -> T
    where
        T: Sum;
    fn tuple<R>(self) -> R
    where
        R: FromVec<T>;
    fn chunk_tuples<R>(self) -> Vec<R>
    where
        R: FromVec<T>;
}

impl<T> SliceExt<T> for [T] {
    fn map_ref<U>(&self, f: impl FnMut(&T) -> U) -> Vec<U> {
        self.iter().map(f).collect()
    }

    fn add<U>(&self, rhs: &[U]) -> Vec<<T as Add<U>>::Output>
    where
        T: Add<U> + Clone,
        U: Clone,
    {
        self.iter()
            .zip(rhs)
            .map(|(lhs, rhs)| lhs.clone() + rhs.clone())
            .collect()
    }

    fn count(&self, mut f: impl FnMut(&T) -> bool) -> usize {
        self.iter().filter(|e| f(e)).count()
    }
}

impl<T> VecExt<T> for Vec<T> {
    fn to_map<V>(self, v: V) -> HashMap<T, V>
    where
        T: Eq + Hash,
        V: Clone,
    {
        self.into_iter().map(|k| (k, v.clone())).collect()
    }

    fn push_new(&mut self, el: T)
    where
        T: PartialEq,
    {
        if !self.contains(&el) {
            self.push(el);
        }
    }

    fn map<U>(self, f: impl FnMut(T) -> U) -> Vec<U> {
        self.into_iter().map(f).collect()
    }

    fn sum(self) -> T
    where
        T: Sum,
    {
        self.into_iter().sum()
    }

    fn tuple<R>(self) -> R
    where
        R: FromVec<T>,
    {
        R::from_vec(self)
    }

    fn chunk_tuples<R>(self) -> Vec<R>
    where
        R: FromVec<T>,
    {
        let res_len = self.len() / R::N;
        let mut iter = self.into_iter();
        let mut res = Vec::new();
        for _ in 0..res_len {
            res.push(R::from_iter(&mut iter));
        }
        res
    }
}

pub fn combs<T: Clone, U: Clone>(a: Vec<T>, b: Vec<U>) -> Vec<(T, U)> {
    a.into_iter()
        .flat_map(|a| b.iter().map(move |b| (a.clone(), b.clone())))
        .collect()
}
