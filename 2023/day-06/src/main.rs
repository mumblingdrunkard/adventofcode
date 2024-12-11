fn ways_to_beat(time: u64, distance: u64) -> u64 {
    let mut ways = 0;
    for i in 1..time {
        let dst = (time - i) * i;
        if dst > distance {
            ways += 1;
        }
    }
    ways
}

fn main() {
    let times = [46, 82, 84, 79];
    let distances = [347, 1522, 1406, 1471];

    let mut prod = 1;
    for (&time, &distance) in times.iter().zip(distances.iter()) {
        prod *= ways_to_beat(time, distance);
    }

    println!("{prod}");

    let (time, distance) = (46828479, 347152214061471);

    println!("{}", ways_to_beat(time, distance));
}
