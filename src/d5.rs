use std::{cmp::Ordering, collections::HashMap};

pub fn p1(s: String) {
    let s_ = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47".to_string();
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut reverse_rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for r in s.split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>()[0].clone().split("\n").map(|x| x.to_string()).collect::<Vec<String>>() {
        let k = r.split("|").collect::<Vec<&str>>().clone()[0].to_string().parse::<i32>().unwrap();
        let v = r.split("|").collect::<Vec<&str>>().clone()[1].to_string().parse::<i32>().unwrap();

        match rules.get_mut(&k) {
            Some(s) => {
                s.push(v);
            },
            None => {
                rules.insert(k, vec![v]);
            }
        }

        match reverse_rules.get_mut(&v) {
            Some(s) => {
                s.push(k);
            },
            None => {
                reverse_rules.insert(v, vec![k]);
            }
        }
    }

    let mut instructions: Vec<Vec<i32>> = vec![];

    for r in s.split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>()[1].clone().split("\n").map(|x| x.to_string()).collect::<Vec<String>>() {
        let step = r.split(",").map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        instructions.push(step);
    }

    // before|next
    let mut count = 0;
    for instruction in instructions {
        if iterate_instruction(instruction.clone(), rules.clone(), reverse_rules.clone()).0 {
            count += instruction[(instruction.len() - 1) / 2];
            // println!("{} : {} : {}", instruction.len(), (instruction.len() / 2) + 1, instruction[(instruction.len() / 2) + 1])
        }
    }
    println!("{count}");
}

pub fn p2(s: String) {
    let s_ = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47".to_string();
    let mut raw_rules: Vec<(i32, i32)> = vec![];
    // let raw_rules: Vec<Vec<i32>> = vec![];

    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut reverse_rules: HashMap<i32, Vec<i32>> = HashMap::new();

    for r in s.split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>()[0].clone().split("\n").map(|x| x.to_string()).collect::<Vec<String>>() {
        let k = r.split("|").collect::<Vec<&str>>().clone()[0].to_string().parse::<i32>().unwrap();
        let v = r.split("|").collect::<Vec<&str>>().clone()[1].to_string().parse::<i32>().unwrap();

        raw_rules.push((k, v));

        match rules.get_mut(&k) {
            Some(s) => { s.push(v); },
            None => { rules.insert(k, vec![v]); }
        }

        match reverse_rules.get_mut(&v) {
            Some(s) => { s.push(k); },
            None => { reverse_rules.insert(v, vec![k]); }
        }
    }

    let mut instructions: Vec<Vec<i32>> = vec![];

    for r in s.split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>()[1].clone().split("\n").map(|x| x.to_string()).collect::<Vec<String>>() {
        let step = r.split(",").map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        instructions.push(step);
    }

    let mut count = 0;
    for instruction in instructions {
        let result: (bool, usize) = iterate_instruction(instruction.clone(), rules.clone(), reverse_rules.clone());
        if result.0 {
            continue;
        }
        let mut broken = instruction.clone();
        broken.sort_by(|a, b| if raw_rules.contains(&(*a, *b)) { Ordering::Less } else { Ordering::Greater });
        count += broken[(broken.len() - 1) / 2];
    }
    println!("{count}");
}

pub fn iterate_instruction(instruction: Vec<i32>, rules: HashMap<i32, Vec<i32>>, reverse_rules: HashMap<i32, Vec<i32>>) -> (bool, usize) {
    let mut passed: Vec<i32> = vec![];
    let mut next: Vec<i32> = instruction.clone();
    for index in 0..instruction.len() {
        let item = next.remove(0);
        for n in &next {
            if !rules.contains_key(&item) {
                break;
            }
            if !rules.get(&item).unwrap().contains(&n) {
                // if it is not in next
                return (false, index);
            }
        }

        for p in &passed {
            // check the back
            if !reverse_rules.contains_key(&item) {
                break;
            }
            if !reverse_rules.get(&item).unwrap().contains(&p) {
                // not in previous
                return (false, index);
            }
        }
        passed.push(item);
    }
    (true, 0)
}