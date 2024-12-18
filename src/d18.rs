use std::collections::HashSet;

pub fn p1(s: String) {
    let mut field: HashSet<(i32, i32)> = HashSet::new();
    for line in s.split("\n").map(|x| x.to_string()) {
        let first = line.split(",").map(|x| x.to_string()).collect::<Vec<String>>().first().unwrap().parse::<i32>().unwrap();
        let last = line.split(",").map(|x| x.to_string()).collect::<Vec<String>>().last().unwrap().parse::<i32>().unwrap();

        field.insert((first, last));

        if field.len() >= 1024 {
            break;
        }
    }

    let h = 71;
    let w = 71;

    let mut traversed: HashSet<(i32, i32)> = HashSet::new();
    traversed.insert((0, 0));

    for y in 0..h {
        let mut result = "".to_string();
        for x in 0..w {
            result.push(if field.contains(&(x, y)) { '#' } else { '.' } );
        }
        println!("{result}");
    }

    let mut nodes = vec![(0, 0)];
    let mut step = 1;
    loop {

        if nodes.is_empty() {
            break;
        }

        let mut next = vec![];
        let mut found = false;
        for n in nodes {
            for neighbour in get_neighbours(n, &field, w, h) {
                if neighbour == (70, 70) {
                    found = true;
                    break;
                }
                if traversed.contains(&neighbour) {
                    continue;
                }
                traversed.insert(neighbour);
                next.push(neighbour);
            }
            if found {
                break;
            }
        }

        if found {
            println!("steps : {step}");
            break;
        }
        nodes = next;
        step += 1;
    }
}

pub fn get_neighbours(p: (i32, i32), f: &HashSet<(i32, i32)>, w: i32, h: i32) -> Vec<(i32, i32)> {
    let mut result = vec![];
    for i in [
        (0, -1),
        (0, 1),
        (1, 0),
        (-1, 0)
    ] {
        let c = (
            i.0 + p.0,
            i.1 + p.1
        );
        if f.contains(&c) {
            continue;
        }

        if (c.0 < 0) || (c.0 >= w) {
            continue;
        }

        if (c.1 < 0) || (c.1 >= h) {
            continue;
        }

        result.push(c);
    }
    result
}

pub fn p2(s: String) {
    let mut field: HashSet<(i32, i32)> = HashSet::new();
    for line in s.split("\n").map(|x| x.to_string()) {
        let first = line.split(",").map(|x| x.to_string()).collect::<Vec<String>>().first().unwrap().parse::<i32>().unwrap();
        let last = line.split(",").map(|x| x.to_string()).collect::<Vec<String>>().last().unwrap().parse::<i32>().unwrap();

        field.insert((first, last));

        if field.len() >= 1024 {
            println!("current : {}", field.len());
            if !try_traverse(field.clone()) {
                println!("{} {:?}", field.len(), (first, last));
                break;
            }
        }
    }
}

pub fn try_traverse(field: HashSet<(i32, i32)>) -> bool {
    let mut traversed: HashSet<(i32, i32)> = HashSet::new();
    traversed.insert((0, 0));


    let h = 71;
    let w = 71;

    let mut nodes = vec![(0, 0)];
    loop {
        if nodes.is_empty() {
            return false;
        }

        let mut next = vec![];
        for n in nodes {
            for neighbour in get_neighbours(n, &field, w, h) {
                if neighbour == (w - 1, h - 1) {
                    return true;
                }
                if traversed.contains(&neighbour) {
                    continue;
                }
                traversed.insert(neighbour);
                next.push(neighbour);
            }
        }

        nodes = next;
    }
}
