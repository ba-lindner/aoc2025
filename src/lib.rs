pub use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};

mod collections;
mod maps;
mod strings;
mod unformat;

pub use collections::*;
pub use maps::*;
pub use strings::*;

pub static TESTING: AtomicBool = AtomicBool::new(false);

pub fn inp() -> String {
    let mut path = std::env::args()
        .nth(1)
        .unwrap_or("inp/test.txt".to_string());
    if path.starts_with("t") || path.starts_with("T") {
        path = "inp/test.txt".to_string();
    } else if let Ok(day) = path.parse::<u8>() {
        path = format!("inp/{day}.txt");
    }
    if path.contains("test") {
        TESTING.store(true, Ordering::Relaxed);
    }
    std::fs::read_to_string(path).unwrap()
}

#[macro_export]
macro_rules! test {
    () => {
        TESTING.load(std::sync::atomic::Relaxed)
    };
    ($test:expr => $real:expr) => {
        if $crate::test!() { $test } else { $real }
    };
}

#[macro_export]
macro_rules! aoc {
    ($part1:ident $(, $part2:ident)? $(as $inp:ty)?) => {
        use $crate::*;

        $(type Data = $inp;)?

        fn main() {
            let data = $crate::inp();
            $(let data: $inp = input(data);)?
            let res = $(if std::env::args().nth(2).is_some() {
                $part2(data).to_string()
            } else )? {
                $part1(data).to_string()
            };
            println!("result = {res}");
        }
    };
}

macro_rules! cmp_macro {
    ($($cmp:ident)*) => {
        $(#[macro_export]
        macro_rules! $cmp {
            ($e:expr) => {
                |e| e.$cmp(&$e)
            };
        })*
    };
}

cmp_macro! [eq ne le lt ge gt];

pub trait BoolExt {
    fn as_sign(self) -> i64;
}

impl BoolExt for bool {
    fn as_sign(self) -> i64 {
        if self { 1 } else { -1 }
    }
}
