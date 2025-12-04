aoc::aoc!(part1, part2 as Map2D<bool>);

fn input(inp: String) -> Data {
    inp.map().to(eq!('@'))
}

fn part1(inp: Data) -> usize {
    inp.pos()
        .filter(|pos| inp[pos])
        .filter(|pos| {
            ALL_DIRS_DIAG.count(|dir| inp.get2d(*pos, *dir).copied().unwrap_or_default()) < 4
        })
        .count()
}

fn part2(mut inp: Data) -> u64 {
    let mut sum = 0;
    let mut changed = true;
    while changed {
        changed = false;
        for pos in inp.pos() {
            if !inp[pos] {
                continue;
            }
            let adj = ALL_DIRS_DIAG.count(|dir| inp.get2d(pos, *dir).copied().unwrap_or_default());
            if adj < 4 {
                sum += 1;
                inp[pos] = false;
                changed = true;
            }
        }
    }
    sum
}
