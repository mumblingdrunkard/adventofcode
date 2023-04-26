use std::collections::HashMap;

struct Dir {
    file: HashMap<String, usize>,
    dir: HashMap<String, Dir>,
}

impl Dir {
    fn create_dir(&mut self, path: &[String], name: String) {
        if path.is_empty() {
            if self.dir.get(&name).is_some() {
                return;
            }

            self.dir.insert(
                name,
                Dir {
                    file: HashMap::new(),
                    dir: HashMap::new(),
                },
            );
        } else {
            self.dir.entry(path[0].clone()).and_modify(|dir| {
                dir.create_dir(&path[1..], name);
            });
        }
    }

    fn insert_file(&mut self, path: &[String], name: String, size: usize) {
        if path.is_empty() {
            if self.file.get(&name).is_some() {
                return;
            }
            self.file.insert(name, size);
        } else {
            self.dir.entry(path[0].clone()).and_modify(|dir| {
                dir.insert_file(&path[1..], name, size);
            });
        }
    }

    fn get_size(&self) -> usize {
        self.file.values().sum::<usize>() + self.dir.values().map(Dir::get_size).sum::<usize>()
    }

    fn get_lim_size(&self, lim: usize, total: &mut usize) -> usize {
        let self_size = self.file.values().sum::<usize>();
        let child_size = self
            .dir
            .values()
            .map(|d| d.get_lim_size(lim, total))
            .sum::<usize>();

        if self_size + child_size <= lim {
            *total += self_size + child_size;
        }

        self_size + child_size
    }

    fn first_lt(&self, lim: usize, result: &mut usize) {
        let size = self.get_size();
        if size <= *result && size >= lim {
            *result = size;
        }

        self.dir.values().for_each(|d| d.first_lt(lim, result));
    }
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let mut fs = Dir {
        file: HashMap::new(),
        dir: HashMap::new(),
    };

    let mut wd: Vec<String> = vec![];

    let mut lines = input.lines();
    let mut current = None;
    lines.next(); // skip first line
    current = lines.next().map(str::to_string);

    'outer: while let Some(line) = current {
        current = None;

        if line.starts_with("$ cd") {
            current = lines.next().map(str::to_string);
            let path = line.split_whitespace().nth(2).unwrap();

            match path {
                ".." => {
                    wd.pop();
                    continue;
                }

                p => wd.push(p.to_string()),
            };
        }

        if line == "$ ls" {
            while let Some(line) = lines.next() {
                if line.starts_with("$") {
                    // done listing dir
                    current = Some(line.to_string());
                    continue 'outer;
                } else if line.starts_with("dir") {
                    // create new directory
                    let name = line.split_whitespace().nth(1).unwrap();
                    fs.create_dir(&wd, name.to_string());
                } else {
                    // insert file
                    let size = line
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                    let name = line.split_whitespace().nth(1).unwrap().to_string();
                    fs.insert_file(&wd, name, size);
                }
            }
        }
    }

    let mut v = 0;
    fs.get_lim_size(100000, &mut v);
    println!("{v}");

    let used_size = fs.get_size();
    let needed = used_size - 40000000;

    let mut v = usize::MAX;
    fs.first_lt(needed, &mut v);
    println!("{v}");
}
