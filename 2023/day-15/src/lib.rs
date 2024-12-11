fn hash(s: &[u8]) -> usize {
    s.iter().map(|&b| b as usize).fold(0usize, |mut cur, v| {
        cur = cur.wrapping_add(v);
        cur = cur.wrapping_mul(v);
        cur %= 256;
        cur
    })
}

pub fn solve<T: AsRef<str>, L: Iterator<Item = T>>(mut lines: L) -> (usize, usize) {
    let line = lines.next().unwrap();
    let part1 = line.as_ref().split(',').map(str::as_bytes).map(hash).sum();

    let mut boxes: Vec<Vec<(Vec<u8>, usize)>> = vec![vec![]; 256];

    for op in line.as_ref().split(',') {
        let op_str = op;
        let op = op.as_bytes();
        if *op.last().unwrap() == b'-' {
            let label = op[..op.len() - 1].to_vec();
            let idx = hash(&label);
            if let Some(pos) = boxes[idx].iter().position(|l| l.0 == label) {
                boxes[idx].remove(pos);
            }
            continue;
        }

        let label = op_str.split('=').next().unwrap().as_bytes().to_vec();
        let idx = hash(&label);
        let len = op_str
            .split('=')
            .nth(1)
            .map(str::parse::<usize>)
            .unwrap()
            .unwrap();

        if let Some(pos) = boxes[idx].iter().position(|l| l.0 == label) {
            boxes[idx][pos].1 = len;
        } else {
            boxes[idx].push((label, len));
        }
    }

    let mut part2 = 0;
    for (box_no, bx) in boxes.iter().enumerate() {
        for (lens_no, (_label, len)) in bx.iter().enumerate() {
            part2 += (box_no + 1) * (lens_no + 1) * *len;
        }
    }

    (part1, part2)
}
