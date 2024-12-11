use std::collections::HashMap;

pub fn p1(s: String) {
    let mut c = s.split(' ').map(|x| x.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    for index in 0..25 {
        println!("{index}");

        let mut next: Vec<i64> = vec![];
        for i in c.clone() {
            if i == 0 {
                next.push(1);
                continue;
            }
            if (i.to_string().len() % 2) == 0 {
                let s = i.to_string();
                let l = s.len();
                let first = s.get(0..(l / 2)).unwrap().to_string().parse::<i64>().unwrap();
                let last = s.get((l / 2)..l).unwrap().to_string().parse::<i64>().unwrap();

                next.push(first);
                next.push(last);
                continue;
            }
            next.push(i * 2024);
        }
        c = next;
    }

    println!("{}", c.len());
}

pub fn p2(s: String) {
    // let s = "125 17".to_string();

    let c = s.split(' ').map(|x| x.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let mut map: HashMap<i64, i64> = HashMap::new();
    for item in &c {
        map.insert(*item, 1);
    }

    for index in 0..75 {
        let mut next: HashMap<i64, i64> = HashMap::new();
        for i in map.clone() {
            if i.0 == 0 {
                match next.get_mut(&1) {
                    Some(n) => { *n += i.1; },
                    None => { next.insert(1, i.1); }
                }
                continue;
            }
            if (i.0.to_string().len() % 2) == 0 {
                let s = i.0.to_string();
                let l = s.len();
                let first = s.get(0..(l / 2)).unwrap().to_string().parse::<i64>().unwrap();
                let last = s.get((l / 2)..l).unwrap().to_string().parse::<i64>().unwrap();

                match next.get_mut(&first) {
                    Some(n) => {
                        *n += i.1;
                    },
                    None => {
                        next.insert(first, i.1);
                    }
                }
                match next.get_mut(&last) {
                    Some(n) => {
                        *n += i.1;
                    },
                    None => {
                        next.insert(last, i.1);
                    }
                }
                continue;
            }
            match next.get_mut(&(i.0 * 2024)) {
                Some(n) => {
                    *n += i.1;
                },
                None => {
                    next.insert(i.0 * 2024, i.1);
                }
            }
        }
        map = next;
    }

    let mut result = 0;
    for m in &map {
        result += m.1;
    }

    println!("result : {}", result);
}

