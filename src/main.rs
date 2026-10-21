fn one_count(byte: u8) -> u8 {
    let mut count = 0u8;
    let mut mask = 1u8;

    for _ in 0..8 {
        if byte & mask != 0 {
            count += 1;
        }
        mask <<= 1;
    }

    count
}

fn sixteenth_sixteenth() {
    let sixteenths: Vec<u8> = (1..=0b1111).collect();
    let mut patterns: Vec<u8> = vec![];

    for i in 0..sixteenths.len() - 1 {
        for j in i + 1..sixteenths.len() {
            patterns.push(sixteenths[i] << 4 | sixteenths[j]);
        }
    }
    patterns.sort();

    let mut patterns_w_count: Vec<(u8, u8)> = patterns
        .into_iter()
        .map(|pattern| (one_count(pattern), pattern))
        .collect();
    patterns_w_count.sort_by_key(|(one_count, _)| *one_count);

    for (_, pattern) in patterns_w_count.iter() {
        println!("{:08b}", pattern);
    }
}

fn triplet_sixteenth() {
    let sixteenths: Vec<u8> = (1..=0b1111).collect();
    let triplets: Vec<u8> = (1..=0b111).collect();

    let mut patterns: Vec<(u8, u8)> = vec![];

    for i in 0..triplets.len() {
        for j in 0..sixteenths.len() {
            patterns.push((triplets[i], sixteenths[j]));
        }
    }
    patterns.sort();

    let mut patterns_w_count: Vec<(u8, (u8, u8))> = patterns
        .into_iter()
        .map(|(triplet, sixteenth)| (one_count(triplet << 4 | sixteenth), (triplet, sixteenth)))
        .collect();
    patterns_w_count.sort_by_key(|(one_count, _)| *one_count);

    for (_, (triplet, sixteenth)) in patterns_w_count.iter() {
        println!("{:03b} {:04b}", triplet, sixteenth);
    }
}

fn triplet_triplet() {
    let triplets: Vec<u8> = (1..=0b111).collect();
    let mut patterns: Vec<(u8, u8)> = vec![];

    for i in 0..triplets.len() - 1 {
        for j in i + 1..triplets.len() {
            patterns.push((triplets[i], triplets[j]));
        }
    }
    patterns.sort();

    let mut patterns_w_count: Vec<(u8, (u8, u8))> = patterns
        .into_iter()
        .map(|(t1, t2)| (one_count(t1 << 3 | t2), (t1, t2)))
        .collect();
    patterns_w_count.sort_by_key(|(one_count, _)| *one_count);

    for (_, (t1, t2)) in patterns_w_count.iter() {
        println!("{:03b} {:03b}", t1, t2);
    }
}

fn main() {
    println!("TRIPLET + TRIPLET");
    println!("---------------------------------------");
    triplet_triplet();
    println!("---------------------------------------");

    println!("TRIPLET + SIXTEENTH");
    println!("---------------------------------------");
    triplet_sixteenth();
    println!("---------------------------------------");

    println!("SIXTEENTH + SIXTEENTH");
    println!("---------------------------------------");
    sixteenth_sixteenth();
    println!("---------------------------------------");
}
