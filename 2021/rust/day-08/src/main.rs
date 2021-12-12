use std::io;

fn count_bits(n: u8) -> usize {
    let mut bits = 0;

    for i in 0..8 {
        if ((n >> i) & 1) == 1 {
            bits += 1;
        }
    }

    bits
}

fn extract(list: &Vec<u8>, bitcount: usize) -> Vec<u8> {
    list.iter()
        .filter(|&&b| bitcount == count_bits(b))
        .map(|&b| b)
        .collect()
}

fn lookup(n: u8, translation: &Vec<u8>) -> i64 {
    translation
        .iter()
        .enumerate()
        .filter(|(_, &v)| v == n)
        .map(|(i, _)| i)
        .next()
        .unwrap() as i64
}

fn decode(input: &Vec<u8>, translation: &Vec<u8>) -> i64 {
    let mut ret = 0;

    for &c in input {
        let v = lookup(c, translation);
        ret += v;
        ret *= 10;
    }

    ret / 10
}

fn decrypt(list: &Vec<u8>) -> Vec<u8> {
    let one = extract(&list, 2)[0];
    let four = extract(&list, 4)[0];
    let seven = extract(&list, 3)[0];
    let eight = extract(&list, 7)[0];

    let len_six = extract(list, 6);
    let len_five = extract(list, 5);

    let three = len_five
        .iter()
        .filter(|&&b| count_bits(b & one) == 2)
        .map(|&b| b)
        .next()
        .unwrap();

    let six = len_six
        .iter()
        .filter(|&&b| count_bits(b & one) == 1)
        .map(|&b| b)
        .next()
        .unwrap();

    let nine = len_six
        .iter()
        .filter(|&&b| count_bits(b & three) == 5)
        .map(|&b| b)
        .next()
        .unwrap();

    let five = six & nine;

    let two = len_five
        .iter()
        .filter(|&&b| count_bits(b & five) == 3)
        .map(|&b| b)
        .next()
        .unwrap();

    let zero = len_six
        .iter()
        .filter(|&&b| count_bits(b & five) == 4)
        .map(|&b| b)
        .next()
        .unwrap();

    [zero, one, two, three, four, five, six, seven, eight, nine].to_vec()
}

fn letters_to_bits(s: &str) -> u8 {
    s.chars()
        .map(|c| match c {
            'a' => 0b0000_0001,
            'b' => 0b0000_0010,
            'c' => 0b0000_0100,
            'd' => 0b0000_1000,
            'e' => 0b0001_0000,
            'f' => 0b0010_0000,
            'g' => 0b0100_0000,
            _ => 0b0000_0000,
        })
        .sum::<u8>()
}

fn main() -> io::Result<()> {
    let mut total = 0;

    loop {
        let mut line = String::new();
        let n = io::stdin().read_line(&mut line)?;
        if n == 0 {
            break;
        }

        let mut line = line.trim().split(" | ");

        let input = line
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| letters_to_bits(s))
            .collect::<Vec<u8>>();

        let output = line
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| letters_to_bits(s))
            .collect::<Vec<u8>>();

        let decrypted = decrypt(&input);

        let decoded = decode(&output, &decrypted);

        total += decoded;

        linenr += 1
    }

    println!("{}", total);

    Ok(())
}
