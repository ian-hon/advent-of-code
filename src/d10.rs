use std::collections::HashSet;

pub fn p1(s: String) {
    let s_ = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732".to_string();

    let mut field: Vec<Vec<i32>> = vec![];

    for (y, row) in s.split("\n").map(|x| x.to_string()).collect::<Vec<String>>().into_iter().enumerate() {
        field.push(vec![]);
        for (_, e) in row.chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>().iter().enumerate() {
            field[y].push(*e);
        }
    }

    let mut starting_points = vec![];
    for (y, row) in field.clone().iter().enumerate() {
        for (x, e) in row.iter().enumerate() {
            if *e != 0 {
                continue;
            }
            starting_points.push((x as usize, y as usize));
        }
    }

    println!("{starting_points:?}");

    let mut result = 0;
    for s in starting_points {
        result += get_score(s, field.clone());
    }
    println!("{result}");
}

pub fn p2(s: String) {
    let s_ = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732".to_string();

    let mut field: Vec<Vec<i32>> = vec![];

    for (y, row) in s.split("\n").map(|x| x.to_string()).collect::<Vec<String>>().into_iter().enumerate() {
        field.push(vec![]);
        for (_, e) in row.chars().map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>().iter().enumerate() {
            field[y].push(*e);
        }
    }

    let mut starting_points = vec![];
    for (y, row) in field.clone().iter().enumerate() {
        for (x, e) in row.iter().enumerate() {
            if *e != 0 {
                continue;
            }
            starting_points.push((x as usize, y as usize));
        }
    }

    println!("{starting_points:?}");

    let mut result = 0;
    for s in starting_points {
        result += get_score_with_repeat(s, field.clone());
    }
    println!("{result}");
}

pub fn get_score(s: (usize, usize), f: Vec<Vec<i32>>) -> i32 {
    let mut traversed: HashSet<(i32, (usize, usize))> = HashSet::new();
    let mut nodes = vec![(0, s)];
    loop {
        let mut next = vec![];
        for node in nodes.clone() {
            for n in get_neighbours(node.0.clone(), node.1.clone(), &f) {
                if traversed.insert((node.0 + 1, n)) {
                    next.push((node.0 + 1, n.clone()));
                }
            }
        }
        if next.len() == 0 {
            break;
        }
        nodes = next;
    }

    let mut result = 0;
    for t in traversed {
        if t.0 == 9 {
            result += 1;
        }
    }
    result
}

pub fn get_neighbours(i: i32, pos: (usize, usize), f: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    if i == 9 {
        return vec![];
    }

    let w = f.len() as i32;
    let h = f[0].len() as i32;
    let pos = (pos.0 as i32, pos.1 as i32);

    let candidates = vec![
        (pos.0, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0 - 1, pos.1)
    ];

    let mut result = vec![];
    for c in candidates {
        if (c.0 < 0) || (c.0 >= w) {
            continue;
        }
        if (c.1 < 0) || (c.1 >= h) {
            continue;
        }
        if f[c.1 as usize][c.0 as usize] == (i + 1) {
            result.push((c.0 as usize, c.1 as usize));
        }
    }

    result
}

pub fn get_score_with_repeat(s: (usize, usize), f: Vec<Vec<i32>>) -> i32 {
    let mut nodes = vec![(0, s)];
    loop {
        let mut next = vec![];
        for node in nodes.clone() {
            for n in get_neighbours(node.0.clone(), node.1.clone(), &f) {
                next.push((node.0 + 1, n.clone()));
            }
        }
        if next.len() == 0 {
            break;
        }
        nodes = next;
    }

    let mut result = 0;
    for i in nodes {
        if i.0 == 9 {
            result += 1;
        }
    }
    result
}

