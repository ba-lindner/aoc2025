use crate::Pos;

pub struct PosIter {
    width: usize,
    height: usize,
    curr_width: usize,
    curr_height: usize,
}

impl PosIter {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            width: x,
            height: y,
            curr_width: 0,
            curr_height: 0,
        }
    }
}

impl Iterator for PosIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_width >= self.width {
            return None;
        }
        let res = (self.curr_width, self.curr_height);
        self.curr_height += 1;
        if self.curr_height >= self.height {
            self.curr_height = 0;
            self.curr_width += 1;
        }
        Some(res)
    }
}
