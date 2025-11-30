use std::collections::VecDeque;

use super::Map2D;

pub struct MapIter<T, I>(pub I)
where
    I: Iterator,
    <I as Iterator>::Item: Iterator<Item = T>;

type Idx<T> = (T, (usize, usize));

impl<T, SI> MapIter<T, SI>
where
    SI: Iterator,
    <SI as Iterator>::Item: Iterator<Item = T>,
{
    pub fn idx(self) -> MapIter<Idx<T>, impl Iterator<Item = impl Iterator<Item = Idx<T>>>> {
        MapIter(
            self.0
                .enumerate()
                .map(|(x, it)| it.enumerate().map(move |(y, t)| (t, (x, y)))),
        )
    }

    pub fn border(self) -> MapIter<T, impl Iterator<Item = impl Iterator<Item = T>>> {
        let mut v: VecDeque<_> = self.0.collect();
        v.pop_back();
        v.pop_front();
        MapIter(v.into_iter().map(|i| {
            let mut dq: VecDeque<_> = i.collect();
            dq.pop_back();
            dq.pop_front();
            dq.into_iter()
        }))
    }

    pub fn find(
        self,
        pos: &mut (usize, usize),
        mut when: impl FnMut(&T) -> bool,
    ) -> MapIter<T, impl Iterator<Item = impl Iterator<Item = T>>> {
        let v: Vec<Vec<_>> = self.0.map(|it| it.collect()).collect();
        'outer: for (x, line) in v.iter().enumerate() {
            for (y, elem) in line.iter().enumerate() {
                if when(elem) {
                    *pos = (x, y);
                    break 'outer;
                }
            }
        }
        MapIter(v.into_iter().map(|r| r.into_iter()))
    }

    pub fn to<U>(self, mut f: impl FnMut(T) -> U) -> Map2D<U> {
        Map2D(self.0.map(|it| it.map(&mut f).collect()).collect())
    }

    pub fn flat<I, U>(self, mut f: impl FnMut(T) -> I) -> Map2D<U>
    where
        I: IntoIterator<Item = U>,
    {
        Map2D(self.0.map(|it| it.flat_map(&mut f).collect()).collect())
    }
}
