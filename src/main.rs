aoc::aoc!(part1, part2 as Map2D<u8>);

fn input(inp: String) -> Data {
    inp.map().to(ctd)
}

fn part1(inp: Data) -> u64 {
    let mut sum = 0;
    for line in &*inp {
        let first = *line[..line.len() - 1].iter().max().unwrap();
        let first_pos = line.iter().position(|&x| x == first).unwrap();
        let second = *line[first_pos + 1..].iter().max().unwrap();
        sum += (first * 10 + second) as u64;
    }
    sum
}

fn part2(inp: Data) -> u64 {
    let mut sum = 0;
    for line in &*inp {
        let mut jolt = 0;
        let mut curr_pos = 0;
        for i in (0..12).rev() {
            let digit = line[curr_pos..line.len() - i].iter().max().unwrap();
            curr_pos += line[curr_pos..line.len() - i]
                .iter()
                .position(|x| x == digit)
                .unwrap()
                + 1;
            jolt = jolt * 10 + (*digit as u64);
        }
        sum += jolt;
    }
    sum
}
