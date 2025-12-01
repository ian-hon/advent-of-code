pub fn p1(s: String) {
    let mut count = 0;
    let mut total = 50;

    for l in s.split("\n") {
        if l.starts_with('R') {
            let l = &l[1..].to_string().parse::<i32>().unwrap();
            total += *l;
        } else {
            let l = &l[1..].to_string().parse::<i32>().unwrap();
            total -= *l;
        }

        total %= 100;

        if total == 0 {
            count += 1;
        }
    }

    println!("{count}");
}

pub fn p2(s: String) {
    // p1 -> 3min
    // p2 -> 4min

    let mut count = 0;
    let mut total = 50;

    for l in s.split("\n") {
        if l.starts_with('R') {
            let l = &l[1..].to_string().parse::<i32>().unwrap();

            for _ in 0..*l {
                total += 1;

                if total >= 100 {
                    total = 0;
                    count += 1;
                }
            }

            println!("R{l} : {count} : {total}");
        } else {
            let l = &l[1..].to_string().parse::<i32>().unwrap();

            for _ in 0..*l {
                total -= 1;

                if total < 0 {
                    total = 99;
                }

                if total == 0 {
                    count += 1;
                }
            }

            println!("L{l} : {count} : {total}");
        }
    }

    println!("{count}");

    //6307 wrong
}
