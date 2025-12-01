aoc::aoc!(part1, part2 as String);

fn input(inp: String) -> Data {
    inp
}

fn part1(inp: Data) -> u64 {
    let mut sum = 0;
    let mut curr = 50;
    for l in inp.lines() {
        let prefix = l.starts_with("R").as_sign();
        let dist: i64 = l[1..].parse().unwrap();
        curr = (curr + 100 + dist * prefix) % 100;
        if curr == 0 {
            sum += 1;
        }
    }
    sum
}

fn part2(inp: Data) -> u64 {
    let mut sum = 0;
    let mut curr = 50;
    for l in inp.lines() {
        let prefix = l.starts_with("R").as_sign();
        let dist = l[1..].parse().unwrap();
        for _ in 0..dist {
            curr = (curr + prefix).rem_euclid(100);
            if curr == 0 {
                sum += 1;
            }
        }
    }
    sum
}
