use std::{fmt::Debug, str::FromStr};

use crate::MapIter;

#[cfg(windows)]
pub const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
pub const LINE_ENDING: &str = "\n";

pub trait StrExt {
    /// split once and unwrap
    fn so<'s>(&'s self, del: &'_ str) -> (&'s str, &'s str);
    /// split, parse, collect into vec
    fn sp<T>(&self, pat: &str) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug;
    /// split, parse, collect into tuple
    fn spt<T, F>(&self, pat: &str) -> F
    where
        F: FromVec<T>,
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        F::from_vec(self.sp(pat))
    }
    /// paragraphs
    fn paras(&self) -> impl Iterator<Item = &str>;
    /// exactly `N` paragraphs
    fn para_arr<const N: usize>(&self) -> [&str; N] {
        self.paras().collect::<Vec<_>>().try_into().unwrap()
    }
    /// start constructing a map
    fn map(&self) -> MapIter<char, impl Iterator<Item = impl Iterator<Item = char>>>;
    /// map lines to vector
    fn ltv<T>(&self, f: impl FnMut(&str) -> T) -> Vec<T>;
    /// extract integer numbers
    fn ints(&self) -> Vec<i64>;
    /// extract unsigned integer numbers
    fn uints(&self) -> Vec<u64>;
}

impl StrExt for str {
    fn so<'s>(&'s self, del: &'_ str) -> (&'s str, &'s str) {
        self.split_once(del).unwrap()
    }

    fn sp<T>(&self, pat: &str) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.split(pat)
            .flat_map(|sl| sl.parse::<T>().ok())
            .collect()
    }

    fn paras(&self) -> impl Iterator<Item = &str> {
        self.split(&format!("{0}{0}", LINE_ENDING))
            .map(|p| p.strip_suffix(LINE_ENDING).unwrap_or(p))
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn map(&self) -> MapIter<char, impl Iterator<Item = impl Iterator<Item = char>>> {
        MapIter(self.lines().map(|l| l.chars()))
    }

    fn ltv<T>(&self, f: impl FnMut(&str) -> T) -> Vec<T> {
        self.lines().map(f).collect()
    }

    fn ints(&self) -> Vec<i64> {
        self.split(|c: char| !c.is_ascii_digit() && c != '-')
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect()
    }

    fn uints(&self) -> Vec<u64> {
        self.split(|c: char| !c.is_ascii_digit())
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect()
    }
}

pub trait FromVec<T>: Sized {
    const N: usize;

    fn from_vec(val: Vec<T>) -> Self {
        Self::from_iter(val.into_iter())
    }

    fn from_iter(iter: impl Iterator<Item = T>) -> Self;
}

macro_rules! first_token {
    ($first:tt $($_:tt)*) => {
        $first
    };
}

macro_rules! impl_from_vec {
    ($n:expr, $first:tt $($t:tt)*) => {
        impl<T> FromVec<T> for (T, $(first_token!(T $t)),*) {
            const N: usize = $n;

            fn from_iter(mut iter: impl Iterator<Item = T>) -> Self {
                (
                    iter.next().unwrap(),
                    $(first_token!(iter $t).next().unwrap()),*
                )
            }
        }
        impl_from_vec!($n - 1, $($t)*);
    };
    ($_:expr,) => {}
}

impl_from_vec!(12, * * * * * * * * * * * *);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ints() {
        let s = "123absd-15acas-689";
        let i = s.ints();
        assert_eq!(&i, &[123, -15, -689]);
    }

    #[test]
    fn uints() {
        let s = "123absd-15acas-689";
        let i = s.uints();
        assert_eq!(&i, &[123, 15, 689]);
    }
}
