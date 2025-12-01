aoc::aoc!(part1, part2 as Vec<(i64, i64)>);

fn input(inp: String) -> Data {
    inp.ltv(|l| (l.starts_with("R").as_sign(), l.ints()[0]))
}

fn part1(inp: Data) -> u64 {
    let mut sum = 0;
    let mut curr = 50;
    for (dir, dist) in inp {
        curr = (curr + dist * dir).rem_euclid(100);
        if curr == 0 {
            sum += 1;
        }
    }
    sum
}

fn part2(inp: Data) -> u64 {
    let mut sum = 0;
    let mut curr = 50;
    for (dir, dist) in inp {
        for _ in 0..dist {
            curr = (curr + dir).rem_euclid(100);
            if curr == 0 {
                sum += 1;
            }
        }
    }
    sum
}
