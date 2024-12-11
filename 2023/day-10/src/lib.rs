#[derive(Clone, Copy, PartialEq, Eq)]
enum Pipe {
    NS,
    NE,
    NW,
    SW,
    SE,
    WE,
    Start,
    Blank,
}

impl Pipe {
    fn from_char(c: char) -> Option<Self> {
        let res = match c {
            '|' => Self::NS,
            '-' => Self::WE,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            'S' => Self::Start,
            _ => None?,
        };
        Some(res)
    }

    fn as_char(&self) -> char {
        match self {
            Self::NS => '│',
            Self::WE => '─',
            Self::NE => '└',
            Self::NW => '┘',
            Self::SW => '┐',
            Self::SE => '┌',
            Self::Blank => ' ',
            _ => panic!(),
        }
    }

    fn explode(&self) -> [[Self; 2]; 2] {
        use Pipe::*;
        match self {
            Self::NS => [[NS, Blank], [NS, Blank]],
            Self::WE => [[WE, WE], [Blank, Blank]],
            Self::NE => [[NE, WE], [Blank, Blank]],
            Self::NW => [[NW, Blank], [Blank, Blank]],
            Self::SW => [[SW, Blank], [NS, Blank]],
            Self::SE => [[SE, WE], [NS, Blank]],
            _ => todo!(),
        }
    }

    fn check(&self, coord: (usize, usize)) -> ((usize, usize), (usize, usize)) {
        let (x, y) = coord;
        match self {
            Pipe::NS => ((x, y - 1), (x, y + 1)),
            Pipe::NE => ((x, y - 1), (x + 1, y)),
            Pipe::NW => ((x, y - 1), (x - 1, y)),
            Pipe::SW => ((x, y + 1), (x - 1, y)),
            Pipe::SE => ((x, y + 1), (x + 1, y)),
            Pipe::WE => ((x - 1, y), (x + 1, y)),
            _ => panic!(),
        }
    }
}

pub fn solve<T: AsRef<str>, L: Iterator<Item = T>>(lines: L) -> (i64, i64) {
    let mut grid = lines
        .map(|line| {
            line.as_ref()
                .chars()
                .map(Pipe::from_char)
                .map(|p| (p, u64::MAX))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut coords = vec![];
    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, (p, _)) in row.iter().enumerate() {
            if let Some(Pipe::Start) = p {
                coords.push((x, y, 0));
                break 'outer;
            }
        }
    }

    // Hardcode this for now
    grid[coords[0].1][coords[0].0] = (Some(Pipe::NS), u64::MAX);
    let mut max_steps = 0;
    while !coords.is_empty() {
        let mut next = vec![];
        for (x, y, steps) in coords.drain(..) {
            let (p, d) = grid[y][x];
            if let Some(p) = p {
                let ((x1, y1), (x2, y2)) = p.check((x, y));
                if d > steps {
                    next.push((x1, y1, steps + 1));
                    next.push((x2, y2, steps + 1));
                    grid[y][x].1 = steps;
                    max_steps = std::cmp::max(steps as i64, max_steps);
                }
            }
        }
        coords = next;
    }

    let s = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|&(p, d)| {
                    if d != u64::MAX {
                        p.unwrap().as_char()
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
                + "\n"
        })
        .collect::<String>();

    let part2 = grid
        .iter()
        .map(|row| {
            let mut inside = false;
            let mut cnt = 0;
            for c in row {
                if c.1 == u64::MAX {
                    cnt += inside as usize;
                    continue;
                }

                if let Some(Pipe::NS | Pipe::SE | Pipe::SW) = c.0 {
                    inside = !inside;
                }
            }
            cnt
        })
        .sum::<usize>();

    (max_steps, part2 as i64)
}
