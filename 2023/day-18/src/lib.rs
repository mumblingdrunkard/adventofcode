fn parse_rgb(rgb: &str) -> (&'static str, isize) {
    let cnt = isize::from_str_radix(&rgb[..5], 16).unwrap();
    let dir = match &rgb[5..] {
        "0" => "R",
        "1" => "D",
        "2" => "L",
        "3" => "U",
        _ => panic!(),
    };

    (dir, cnt)
}

pub fn solve<T: AsRef<str>, L: Iterator<Item = T>>(lines: L) -> (usize, usize) {
    let (mut x, mut y) = (0, 0);
    let mut two_a = 0;
    let mut p1sum = 0;

    let (mut u, mut v) = (0, 0);
    let mut two_b = 0;
    let mut p2sum = 0;

    for line in lines {
        let mut split = line.as_ref().split_whitespace();
        let dir = split.next().unwrap();
        let cnt = split.next().unwrap().parse::<isize>().unwrap();

        let (nx, ny) = match dir {
            "R" => (x + cnt, y),
            "L" => (x - cnt, y),
            "U" => (x, y - cnt),
            "D" => (x, y + cnt),
            _ => panic!(),
        };
        two_a += x * ny - y * nx;
        p1sum += cnt;
        (x, y) = (nx, ny);

        let rgb = split
            .next()
            .unwrap()
            .trim_matches(|c| ['(', ')', '#'].contains(&c));

        let (dir, cnt) = parse_rgb(rgb);
        let (nu, nv) = match dir {
            "R" => (u + cnt, v),
            "L" => (u - cnt, v),
            "U" => (u, v - cnt),
            "D" => (u, v + cnt),
            _ => panic!(),
        };
        two_b += u * nv - v * nu;
        p2sum += cnt;
        (u, v) = (nu, nv);
    }

    let part1 = (two_a - p1sum) / 2 + p1sum + 1;
    let part2 = (two_b - p2sum) / 2 + p2sum + 1;

    (part1 as usize, part2 as usize)
}
