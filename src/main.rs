aoc::aoc!(part1, part2 as Vec<(u64, u64)>);

fn input(inp: String) -> Data {
    inp.uints().chunk_tuples()
}

fn part1(inp: Data) -> u64 {
    let mut sum = 0;
    for (l, r) in inp {
        for i in l..=r {
            let s = i.to_string();
            if s[..s.len() / 2] == s[s.len() / 2..] {
                sum += i;
            }
        }
    }
    sum
}

fn part2(inp: Data) -> u64 {
    let mut sum = 0;
    for (l, r) in inp {
        for i in l..=r {
            let s = i.to_string();
            for p in 1..=s.len() / 2 {
                if s == s[..p].repeat(s.len() / p) {
                    sum += i;
                    break;
                }
            }
        }
    }
    sum
}
