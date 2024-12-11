use cached::proc_macro::cached;

#[cached(
    key = "(u8, u8, u16)",
    convert = r#"{(states.len() as u8, groups.len() as u8, id)}"#
)]
fn count(states: &[u8], groups: &[usize], id: u16) -> usize {
    if groups.is_empty() {
        if states.iter().any(|&s| s == b'#') {
            return 0;
        }
        return 1;
    }

    let len = groups[0];
    let mut sum = 0;
    for i in 0..states.len() {
        if states[i..].len() < len || i > 0 && states[i - 1] == b'#' {
            break;
        }
        if states[i..i + len].iter().any(|&s| s == b'.')
            || states[i..].len() > len && states[i + len] == b'#'
        {
            continue;
        }

        if states[i..].len() > len {
            sum += count(&states[i + len + 1..], &groups[1..], id);
        } else if groups.len() == 1 {
            sum += 1;
        }
    }

    sum
}

fn solve_part1(states_str: &str, groups: &[usize], lineno: u16) -> usize {
    count(states_str.as_bytes(), groups, lineno * 2)
}

fn solve_part2(states_str: &str, groups: &[usize], lineno: u16) -> usize {
    let states = [states_str.as_bytes()]
        .into_iter()
        .map(<[u8]>::to_owned)
        .cycle()
        .take(5)
        .collect::<Vec<_>>()
        .join(&b'?');
    let groups = groups.repeat(5);
    count(&states, &groups, lineno * 2 + 1)
}

pub fn solve<T: AsRef<str>, L: Iterator<Item = T>>(lines: L) -> (i64, i64) {
    let (mut part1, mut part2) = (0, 0);

    for (lineno, line) in lines.enumerate() {
        let states = line.as_ref().split_whitespace().next().unwrap();

        let groupings = line.as_ref().split_whitespace().nth(1).unwrap();
        let groupings = groupings
            .split(',')
            .map(str::parse::<usize>)
            .map_while(Result::ok)
            .collect::<Vec<_>>();

        part1 += solve_part1(states, &groupings, lineno as u16);
        part2 += solve_part2(states, &groupings, lineno as u16);
    }

    (part1 as i64, part2 as i64)
}
