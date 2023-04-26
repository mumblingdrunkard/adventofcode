use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(i64),
    Mul(i64),
    Square,
}

impl Operation {
    pub fn apply(&self, old: i64) -> i64 {
        match self {
            Operation::Add(val) => (old + val) / 3,
            Operation::Mul(val) => (old * val) / 3,
            Operation::Square => (old * old) / 3,
        }
    }

    pub fn apply_z(&self, old: i64, z: i64) -> i64 {
        match self {
            Operation::Add(val) => (old + val) % z,
            Operation::Mul(val) => (old * val) % z,
            Operation::Square => (old * old) % z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Test {
    pub divisor: i64,
    pass: usize,
    fail: usize,
}

impl Test {
    pub fn apply(&self, item: i64) -> usize {
        match item % self.divisor == 0 {
            true => self.pass,
            _ => self.fail,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: Vec<i64>,
    pub operation: Operation,
    pub test: Test,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_monkey(s).ok_or(())
    }
}

fn parse_monkey(s: &str) -> Option<Monkey> {
    let mut lines = s.lines();

    // parse the id
    let _ = lines.next()?;

    // parse the items
    let items = lines
        .next()?
        .split_once(": ")
        .map(|(_, items)| items)?
        .split(", ")
        .map(str::parse::<i64>)
        .map(Result::ok)
        .collect::<Option<Vec<_>>>()?;

    // parse the operation
    let mut operation_parts = lines
        .next()?
        .split_once(" = ")
        .map(|(_, operation)| operation)?
        .split_whitespace()
        .skip(1);

    let operation = {
        let operator = operation_parts.next()?;
        let value = operation_parts.next()?;

        match (value, operator) {
            ("old", "*") => Operation::Square,
            ("old", "+") => Operation::Mul(2),
            (val, "*") => Operation::Mul(val.parse().ok()?),
            (val, "+") => Operation::Add(val.parse().ok()?),
            _ => None?,
        }
    };

    // parse the test
    let divisor = lines
        .next()?
        .split_once(" by ")
        .map(|(_, divisor)| divisor)?
        .parse::<i64>()
        .ok()?;

    let pass = lines
        .next()?
        .split_once(" monkey ")
        .map(|(_, target)| target)?
        .parse::<usize>()
        .ok()?;

    let fail = lines
        .next()?
        .split_once(" monkey ")
        .map(|(_, target)| target)?
        .parse::<usize>()
        .ok()?;

    let test = Test {
        divisor,
        pass,
        fail,
    };

    // finally build this fucking monkey
    Some(Monkey {
        items,
        operation,
        test,
    })
}
