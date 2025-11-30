use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut, Index, IndexMut},
};

mod creation;
mod dirs;
mod iter;

pub use creation::*;
pub use dirs::*;

use crate::{VecExt, maps::iter::PosIter};

pub type Pos = (usize, usize);

pub struct Map2D<T>(pub Vec<Vec<T>>);

impl<T> Map2D<T> {
    pub fn new(value: T, size_x: usize, size_y: usize) -> Self
    where
        T: Clone,
    {
        Self(vec![vec![value; size_y]; size_x])
    }

    pub fn sq(value: T, size: usize) -> Self
    where
        T: Clone,
    {
        Self::new(value, size, size)
    }

    pub fn transpose(self) -> Self {
        let mut res: Vec<Vec<T>> = std::iter::repeat_with(|| Vec::with_capacity(self.len()))
            .take(self[0].len())
            .collect();
        for line in self.0 {
            for (idx, elem) in line.into_iter().enumerate() {
                res[idx].push(elem);
            }
        }
        Map2D(res)
    }
}

impl<T> Deref for Map2D<T> {
    type Target = Vec<Vec<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Map2D<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Index<Pos> for Map2D<T> {
    type Output = T;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<T> IndexMut<Pos> for Map2D<T> {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

impl<T> Index<usize> for Map2D<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Map2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, I> Index<&I> for Map2D<T>
where
    Map2D<T>: Index<I>,
    I: Clone,
{
    type Output = <Map2D<T> as Index<I>>::Output;

    fn index(&self, index: &I) -> &Self::Output {
        self.index(index.clone())
    }
}

impl<T, I> IndexMut<&I> for Map2D<T>
where
    Map2D<T>: IndexMut<I>,
    I: Clone,
{
    fn index_mut(&mut self, index: &I) -> &mut Self::Output {
        self.index_mut(index.clone())
    }
}

pub trait MapExt<T>
where
    Self: IndexMut<usize>,
    <Self as Index<usize>>::Output: BorrowMut<[T]>,
{
    fn size(&self) -> (usize, usize);
    fn add_dir(&self, pos: Pos, dir: (i8, i8)) -> Option<Pos> {
        let (max_x, max_y) = self.size();
        let x: usize = (pos.0 as isize + dir.0 as isize).try_into().ok()?;
        let y: usize = (pos.1 as isize + dir.1 as isize).try_into().ok()?;
        (x < max_x && y < max_y).then_some((x, y))
    }
    fn add_dir_n(&self, pos: Pos, dir: (i8, i8), n: usize) -> Option<Pos> {
        let mut p = pos;
        for _ in 0..n {
            p = self.add_dir(p, dir)?;
        }
        Some(p)
    }
    fn get2d(&self, pos: Pos, dir: (i8, i8)) -> Option<&T> {
        let (x, y) = self.add_dir(pos, dir)?;
        Some(&self[x].borrow()[y])
    }
    fn get2d_mut(&mut self, pos: Pos, dir: (i8, i8)) -> Option<&mut T> {
        let (x, y) = self.add_dir(pos, dir)?;
        Some(&mut self[x].borrow_mut()[y])
    }
    fn count2d(&self, f: impl FnMut(&T) -> bool) -> usize;
    fn adj(&self, pos: Pos) -> Vec<Pos> {
        ALL_DIRS
            .into_iter()
            .flat_map(|dir| self.add_dir(pos, dir))
            .collect()
    }
    fn pos(&self) -> PosIter {
        let (x, y) = self.size();
        PosIter::new(x, y)
    }
}

impl<T> MapExt<T> for [Vec<T>] {
    fn size(&self) -> (usize, usize) {
        (self.len(), self[0].len())
    }

    fn count2d(&self, mut f: impl FnMut(&T) -> bool) -> usize {
        self.iter().map(|l| l.count(&mut f)).sum()
    }
}

impl<T, const N: usize> MapExt<T> for [[T; N]] {
    fn size(&self) -> (usize, usize) {
        (self.len(), N)
    }

    fn count2d(&self, mut f: impl FnMut(&T) -> bool) -> usize {
        self.iter()
            .map(|l| l.iter().filter(|el| f(el)).count())
            .sum()
    }
}
