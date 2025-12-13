pub fn p1(s: String) {
    //     let s = "3-5
    // 10-14
    // 16-20
    // 12-18

    // 1
    // 5
    // 8
    // 11
    // 17
    // 32";

    let mut ss = s.split("\n\n");
    let ranges = ss
        .next()
        .unwrap()
        .lines()
        .map(|x| x.split("-").collect::<Vec<&str>>())
        .map(|v| v[0].parse::<i64>().unwrap()..=v[1].parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let candidates = ss
        .next()
        .unwrap()
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!(
        "{}",
        candidates
            .iter()
            .filter(|x| ranges.iter().any(|i| i.contains(x)))
            .count()
    );
}

pub fn p2(s: String) {
    let _s = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    let mut events = s
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(|x| {
            (
                x.split("-").collect::<Vec<&str>>()[0]
                    .parse::<u64>()
                    .unwrap(),
                x.split("-").collect::<Vec<&str>>()[1]
                    .parse::<u64>()
                    .unwrap(),
            )
        })
        .collect::<Vec<(u64, u64)>>();
    events.sort_by_key(|x| x.0);
    events.push((u64::MAX, u64::MAX));

    let mut total = 0;
    let mut current = (events[0].0, events[0].1);
    for (low, high) in &events[1..] {
        println!("{low}-{high} : {current:?}");
        if *low <= current.1 {
            current.1 = (*high).max(current.1);
            // current.1 = *high;
            println!("\tassigned");
        } else {
            total += current.1 - current.0 + 1;
            println!("adding : {}", current.1 - current.0 + 1);
            current = (*low, *high);
        }
    }
    println!("total: {total}");
}
