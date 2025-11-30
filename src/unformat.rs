#[macro_export]
macro_rules! unformat {
    ($fnname:ident => $($start:literal)? $({$var:ident : $t:ty} $sep:literal)+) => {
        #[allow(unused)]
        pub fn $fnname(line: &str) -> ($($t),+) {
            let line = line.replace("\r\n", "\n");
            $(let line = line.strip_prefix($start).unwrap();)?
            let tail: &str = &line;
            $(
                let ($var, tail) = if $sep == "" {
                    (tail, "")
                } else {
                    tail.split_once($sep).unwrap()
                };
            )+
            ($(
                $crate::process!($var, $t)
            ),+)
        }
    };
}

#[macro_export]
macro_rules! process {
    ($val:ident, String) => {
        $val.to_string()
    };
    ($val:ident, $t:ty) => {
        $val.parse::<$t>().unwrap()
    };
}

#[cfg(test)]
mod test {
    unformat!(f_splitlike => {a:u32}"--"{b:u32}"");

    #[test]
    fn splitlike() {
        assert_eq!(f_splitlike("1--2"), (1, 2));
    }

    unformat!(f_newline => "start at "{start:u32}",
then go to "{mid:u32}"
and finally to "{end:u32}"");

    #[test]
    fn newline() {
        assert_eq!(
            "
",
            "\n"
        );
        assert_eq!(
            f_newline("start at 123,\r\nthen go to 456\r\nand finally to 789"),
            (123, 456, 789)
        );
    }
}
