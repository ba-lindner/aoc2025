pub type Dir = (i8, i8);

// parsed map: +x = south, -x = north, +y = east, -y = west
pub const DIR_NORTH_PARSED: Dir = (-1, 0);
pub const DIR_NORTH_TRUE: Dir = (0, 1);

pub const ALL_DIRS: [Dir; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
pub const ALL_DIRS_DIAG: [Dir; 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

pub trait DirExt {
    fn left(&self) -> Self;
    fn right(&self) -> Self;
    fn rl(&mut self)
    where
        Self: Sized,
    {
        *self = self.left();
    }
    fn rr(&mut self)
    where
        Self: Sized,
    {
        *self = self.right();
    }

    fn idx(&self) -> usize;
}

impl DirExt for Dir {
    fn left(&self) -> Self {
        (-self.1, self.0)
    }

    fn right(&self) -> Self {
        (self.1, -self.0)
    }

    fn idx(&self) -> usize {
        ALL_DIRS.iter().position(|d| d == self).unwrap()
    }
}

pub fn ctd(c: char) -> u8 {
    c as u8 - b'0'
}

pub fn parsed_dir(c: char) -> Option<Dir> {
    Some(match c {
        '^' => DIR_NORTH_PARSED,
        '>' => DIR_NORTH_PARSED.right(),
        '<' => DIR_NORTH_PARSED.left(),
        'v' => DIR_NORTH_PARSED.left().left(),
        _ => None?,
    })
}

pub fn true_dir(c: char) -> Option<Dir> {
    Some(match c {
        '^' => DIR_NORTH_TRUE,
        '>' => DIR_NORTH_TRUE.right(),
        '<' => DIR_NORTH_TRUE.left(),
        'v' => DIR_NORTH_TRUE.left().left(),
        _ => None?,
    })
}
