pub fn p1(s: String) {
    //     let s = "..@@.@@@@.
    // @@@.@.@.@@
    // @@@@@.@.@@
    // @.@@@@..@.
    // @@.@@@@.@@
    // .@@@@@@@.@
    // .@.@.@.@@@
    // @.@@@.@@@@
    // .@@@@@@@@.
    // @.@.@@@.@.";

    // 140
    let mut map = s
        .lines()
        .map(|l| l.chars().map(|x| (x == '@') as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    map.insert(0, vec![0; 142]);
    for l in &mut map {
        l.push(0);
        l.insert(0, 0);
    }
    map.push(vec![0; 142]);

    let mut count = 0;
    for x in 1..=140_usize {
        for y in 1..=140_usize {
            if map[y][x] == 0 {
                continue;
            }
            if map[y - 1..=y + 1]
                .iter()
                .flat_map(|v| v[x - 1..=x + 1].iter())
                .sum::<u8>()
                < 5
            {
                count += 1;
            }
        }
    }

    println!("count: {count}");
}

pub fn p2(s: String) {
    //     let s = "..@@.@@@@.
    // @@@.@.@.@@
    // @@@@@.@.@@
    // @.@@@@..@.
    // @@.@@@@.@@
    // .@@@@@@@.@
    // .@.@.@.@@@
    // @.@@@.@@@@
    // .@@@@@@@@.
    // @.@.@@@.@.";

    // 140
    let mut map = s
        .lines()
        .map(|l| l.chars().map(|x| (x == '@') as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    map.insert(0, vec![0; 142]);
    for l in &mut map {
        l.push(0);
        l.insert(0, 0);
    }
    map.push(vec![0; 142]);

    let mut total = 0;
    loop {
        // let mut flag = false;
        let mut current = map.clone();
        let mut count = 0;
        for x in 1..=140_usize {
            for y in 1..=140_usize {
                if map[y][x] == 0 {
                    continue;
                }
                if map[y - 1..=y + 1]
                    .iter()
                    .flat_map(|v| v[x - 1..=x + 1].iter())
                    .sum::<u8>()
                    < 5
                {
                    count += 1;
                    current[y][x] = 0;
                }
            }
        }
        total += count;
        map = current;
        if count == 0 {
            break;
        }
    }
    println!("total: {total}");
}
