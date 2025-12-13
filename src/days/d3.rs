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
    // dfs backtrack?
}
