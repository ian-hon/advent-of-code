pub fn p1(s: String) {
    // let s = "987654321111111
    // 811111111111119
    // 234234234234278
    // 818181911112111";

    let mut total = 0;
    for line in s.split("\n") {
        let length = line.len();

        let mut highest: Option<(usize, i32)> = None;
        let mut p_highest: Option<(usize, i32)> = None;
        for (index, c) in line.chars().enumerate() {
            let c = c.to_digit(10).unwrap() as i32;

            match highest {
                Some((i, v)) => {
                    if c > v {
                        p_highest = Some((i, v));
                        highest = Some((index, c));
                    }
                }
                None => {
                    highest = Some((index, c));
                }
            }
        }

        let highest = highest.unwrap();
        // if at end, use previous
        // if not at end, traverse onwards

        if (length - 1) == highest.0 {
            // at end
            let p_highest = p_highest.unwrap();
            total += (p_highest.1 * 10) + highest.1;
            println!("{line} {}", (p_highest.1 * 10) + highest.1);
        } else {
            // not end
            // traverse

            let mut h = 0;
            for i in (highest.0 + 1)..length {
                let value = line
                    .chars()
                    .collect::<Vec<char>>()
                    .get(i)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as i32;
                if value > h {
                    h = value;
                }
            }
            total += (highest.1 * 10) + h;
            println!("{line} {}", (highest.1 * 10) + h);
        }
    }
    println!("total: {total}");
}

pub fn p2(s: String) {
    //     let s = "987654321111111
    // 811111111111119
    // 234234234234278
    // 818181911112111";

    // dfs backtrack?
    let mut total = 0;
    for line in s.lines().map(|x| {
        x.chars()
            .map(|y| y.to_digit(10).unwrap() as u8)
            .collect::<Vec<u8>>()
    }) {
        let length = line.len();
        let mut current = 0;

        let mut previous = 0;
        for i in 0..12 {
            let ending = length - (11 - i);
            let mut to_search = line[previous..ending]
                .iter()
                .copied()
                .enumerate()
                .collect::<Vec<(usize, u8)>>();
            to_search.sort_by_key(|x| (x.1 as u64 * 1000) - (x.0 as u64));
            let last = to_search.last().unwrap().clone();

            previous = last.0 + 1 + previous;

            current += (last.1 as u64) * (10_u64.pow(11 - (i as u32)));
        }
        println!("{line:?} current: {current}");
        total += current;
    }
    println!("total: {total}");
}
