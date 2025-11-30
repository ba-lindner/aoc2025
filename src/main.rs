aoc::aoc!(part1, part2 as String);

fn input(inp: String) -> Data {
    inp
}

fn part1(inp: Data) -> i64 {
    *inp.ltv(|l| l.ints().sum()).iter().max().unwrap()
}

fn part2(inp: Data) -> u64 {
    *inp.ltv(|l| l.uints().sum()).iter().min().unwrap()
}
