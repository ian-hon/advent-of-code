use std::collections::HashSet;

pub fn p1(s: String) {
    // let s = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let mut total: i64 = 0;
    for line in s.split(",") {
        let l = line.split("-").collect::<Vec<&str>>();
        let lower = l[0].parse::<i64>().unwrap();
        let upper = l[1].parse::<i64>().unwrap();

        // println!("{lower} : {upper}");

        for number in lower..=upper {
            // only works if number of digits are even
            let digits = number.to_string().len();
            if digits % 2 != 0 {
                continue;
            }

            let offset = 10i64.pow((digits / 2) as u32);

            let left = number / offset;
            let right = number - (left * offset);

            if left == right {
                total += number;
            }
        }
    }
    println!("total: {total}");
}

pub fn p2(s: String) {
    // let s = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    // take largest potential value
    // brute force one by one potential ids
    let mut total: i64 = 0;
    for line in s.split(",") {
        let l = line.split("-").collect::<Vec<&str>>();
        let lower = l[0].parse::<i64>().unwrap();
        let upper = l[1].parse::<i64>().unwrap();

        let half_digits = ((upper.to_string().len() as f64) / 2_f64).floor() as i32;
        let highest = (upper as i64) / ((10_i64.pow(half_digits as u32)) as i64);

        println!("{lower}-{upper} : {half_digits} : {highest}");

        let mut map: HashSet<i64> = HashSet::new();
        for candidate in 1..=highest {
            let mut b = candidate.to_string();
            'inner: while b.is_empty() || (b.parse::<i64>().unwrap() <= upper) {
                b.push_str(&candidate.to_string());
                let c = b.parse::<i64>().unwrap();

                if (lower..=upper).contains(&c) {
                    if map.contains(&c) {
                        continue 'inner;
                    }

                    map.insert(c);
                    println!("\t\tvalid: {c}");
                    total += c;
                }
            }
        }
    }
    println!("total: {total}");
}
