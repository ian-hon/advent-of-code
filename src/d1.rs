use std::collections::HashMap;

pub fn p1(s: String) {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    for i in s.split('\n') {
        let t = i.to_string().split(" ").map(|x| x.to_string()).filter(|x| x != "").collect::<Vec<String>>();
        left.push(t[0].parse::<i32>().unwrap());
        right.push(t[1].parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut distance = 0;

    for i in 0..left.len() {
        distance += (left[i] - right[i]).abs()
    }

    println!("{distance}");
}

pub fn p2(s: String) {
    let mut left: Vec<i32> = vec![];
    let mut right: HashMap<i32, i32> = HashMap::new();
    for i in s.split('\n') {
        let t = i.to_string().split(" ").map(|x| x.to_string()).filter(|x| x != "").collect::<Vec<String>>();
        left.push(t[0].parse::<i32>().unwrap());

        let r = t[1].parse::<i32>().unwrap();

        if !right.contains_key(&r) {
            right.insert(r, 1);
        } else {
            *right.get_mut(&r).unwrap() += 1;
        }
    }

    let mut score = 0;

    for i in left {
        match right.get(&i) {
            Some(n) => {
                score += *n * i;

                println!("{i} : {}", *n * i);
            },
            None => {}
        }
    }

    println!("{score}");
}