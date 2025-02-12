pub fn p1(s: String) {
    let _s = "1
10
100
2024".to_string();

    let mut numbers = vec![];
    for i in s.split("\n") {
        numbers.push(i.to_string().parse::<i128>().unwrap());
    }

    for n in &mut numbers {
        for _ in 0..2000 {
            *n = generate(*n);
        }
    }

    println!("{}", numbers.iter().sum::<i128>());
}

pub fn generate(n: i128) -> i128 {
    let mut f = n;

    f = mix(f << 6, f);
    f = prune(f);

    f = mix(f >> 5, f);
    f = prune(f);

    f = mix(f << 11, f);
    f = prune(f);

    f
}

pub fn mix(i: i128, s: i128) -> i128 {
    i ^ s
}

pub fn prune(i: i128) -> i128 {
    i & 16777215
}
