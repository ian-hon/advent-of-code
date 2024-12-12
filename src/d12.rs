use std::{collections::{HashMap, HashSet}, hash::Hash};

pub fn p1(s: String) {
    let s_ = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE".to_string();
    let mut field: Vec<Vec<char>> = vec![];
    for (y, row) in s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>().iter().enumerate() {
        field.push(vec![]);
        for e in row.chars() {
            field[y].push(e);
        }
    }

    let h = field.len() as i32;
    let w = field[0].len() as i32;

    let mut traversed: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: Vec<HashSet<(usize, usize)>> = vec![];
    for (y, row) in field.clone().iter().enumerate() {
        for (x, starting) in row.iter().enumerate() {
            if traversed.contains(&(x, y)) {
                continue;
            }

            let mut r: HashSet<(usize, usize)> = HashSet::new();
            let mut nodes: HashSet<(usize, usize)> = HashSet::new();
            nodes.insert((x, y));
            r.insert((x, y));
            loop {
                if nodes.is_empty() {
                    break;
                }

                let mut next: HashSet<(usize, usize)> = HashSet::new();
                for node in &nodes {
                    for neighbour in get_neighbours(node.clone(), *starting, &field, &traversed, w, h) {
                        next.insert(neighbour);
                        traversed.insert(neighbour);
                        r.insert(neighbour);
                    }
                }
                nodes = next;
            }
            regions.push(r);
        }
    }

    let mut result = 0;
    for r in regions {
        let mut area = 0;
        let mut perimeter = 0;
        for e in r.clone() {
            area += 1;
            let candidates = vec![
                (e.0 as i32 + 1, e.1 as i32),
                (e.0 as i32 - 1, e.1 as i32),
                (e.0 as i32, e.1 as i32 + 1),
                (e.0 as i32, e.1 as i32 - 1)
            ];

            for c in candidates {
                if c.0 < 0 {
                    perimeter += 1;
                    continue;
                }
                if c.0 >= w {
                    perimeter += 1;
                    continue;
                }
        
                if c.1 < 0 {
                    perimeter += 1;
                    continue;
                }
                if c.1 >= h {
                    perimeter += 1;
                    continue;
                }

                if !r.contains(&(c.0 as usize, c.1 as usize)) {
                    perimeter += 1;
                }
            }
        }
        result += area * perimeter;
    }
    println!("{result:?}");
}

pub fn get_neighbours(p: (usize, usize), c: char, m: &Vec<Vec<char>>, t: &HashSet<(usize, usize)>, w: i32, h: i32) -> Vec<(usize, usize)> {
    let candidates = vec![
        (p.0 as i32, p.1 as i32 + 1),
        (p.0 as i32, p.1 as i32 - 1),
        (p.0 as i32 + 1, p.1 as i32),
        (p.0 as i32 - 1, p.1 as i32)
    ];

    let mut result = vec![];
    for candidate in candidates {
        if candidate.0 < 0 {
            continue;
        }
        if candidate.0 >= w {
            continue;
        }

        if candidate.1 < 0 {
            continue;
        }
        if candidate.1 >= h {
            continue;
        }

        if t.contains(&(candidate.0 as usize, candidate.1 as usize)) {
            continue;
        }

        if m[candidate.1 as usize][candidate.0 as usize] != c {
            continue;
        }

        result.push((candidate.0 as usize, candidate.1 as usize));
    }

    result
}

pub fn p2(s: String) {
    let s_ = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE".to_string();
    let mut field: Vec<Vec<char>> = vec![];
    for (y, row) in s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>().iter().enumerate() {
        field.push(vec![]);
        for e in row.chars() {
            field[y].push(e);
        }
    }

    let h = field.len() as i32;
    let w = field[0].len() as i32;

    let mut traversed: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: Vec<HashSet<(usize, usize)>> = vec![];
    for (y, row) in field.clone().iter().enumerate() {
        for (x, starting) in row.iter().enumerate() {
            if traversed.contains(&(x, y)) {
                continue;
            }

            let mut r: HashSet<(usize, usize)> = HashSet::new();
            let mut nodes: HashSet<(usize, usize)> = HashSet::new();
            nodes.insert((x, y));
            r.insert((x, y));
            loop {
                if nodes.is_empty() {
                    break;
                }

                let mut next: HashSet<(usize, usize)> = HashSet::new();
                for node in &nodes {
                    for neighbour in get_neighbours(node.clone(), *starting, &field, &traversed, w, h) {
                        next.insert(neighbour);
                        traversed.insert(neighbour);
                        r.insert(neighbour);
                    }
                }
                nodes = next;
            }
            regions.push(r);
        }
    }

    let mut result = 0;
    for r in regions {
        let mut area = 0;
        let mut perimeter = 0;
        let mut edge_map: Vec<HashSet<(usize, usize)>> = vec![HashSet::new(), HashSet::new(), HashSet::new(), HashSet::new()];
        for e in r.clone() {
            area += 1;
            let candidates = vec![
                (e.0 as i32, e.1 as i32 - 1),
                (e.0 as i32 + 1, e.1 as i32),
                (e.0 as i32, e.1 as i32 + 1),
                (e.0 as i32 - 1, e.1 as i32)
            ];

            for (t, c) in candidates.iter().enumerate() {
                if c.0 < 0 {
                    edge_map[3].insert(e.clone());
                    continue;
                }
                if c.0 >= w {
                    edge_map[1].insert(e.clone());
                    continue;
                }
        
                if c.1 < 0 {
                    edge_map[0].insert(e.clone());
                    continue;
                }
                if c.1 >= h {
                    edge_map[2].insert(e.clone());
                    continue;
                }

                if r.contains(&(c.0 as usize, c.1 as usize)) {
                    continue;
                }

                edge_map[t].insert(e.clone());

                // if search_edge(t, &edge_map, c.clone(), w, h) {

                // } else {
                //     // add to edge map
                //     edge_map[t].insert((c.0 as usize, c.1 as usize));
                //     perimeter += 1;
                // }
            }
        }
        
        for (t, edges) in edge_map.iter().enumerate() {
            // let edges = edges.iter().map(|x| (x.0, x.1)).collect::<Vec<(usize, usize)>>();
            let mut grouped_edges: HashMap<usize, Vec<usize>> = HashMap::new();

            if t % 2 == 1 {
                // vertical one
                // let edges = edges.iter().map(|x| )
                for e in edges {
                    match grouped_edges.get_mut(&e.0) {
                        Some(c) => {
                            c.push(e.1);
                        },
                        None => {
                            grouped_edges.insert(e.0, vec![e.1]);
                        }
                    }
                }
            } else {
                for e in edges {
                    match grouped_edges.get_mut(&e.1) {
                        Some(c) => {
                            c.push(e.0);
                        },
                        None => {
                            grouped_edges.insert(e.1, vec![e.0]);
                        }
                    }
                }
            }

            let mut count = 0;
            for item in &grouped_edges {
                let mut iterator = item.1.clone();
                iterator.sort();

                let mut iterator = iterator.iter();
                let mut previous = *iterator.next().unwrap();
                while let Some(n) = iterator.next() {
                    if (*n - previous) != 1 {
                        count += 1;
                    }

                    previous = *n;
                }

                count += 1;
            }
            perimeter += count;
        }
        result += area * perimeter;
    }
    println!("{result:?}");
}

pub fn search_edge(t: usize, collection: &Vec<HashSet<(usize, usize)>>, pos: (i32, i32), w: i32, h: i32) -> bool {
    // true if theres another connecting edge
    // if even -> vertical (up and down)
    let increment = if t % 2 == 0 { vec![(0, 1), (0, -1)] } else { vec![(1, 0), (-1, 0)] };
    let starting = pos;
    for i in increment {
        let mut pos = starting;
        loop {
            pos.0 += i.0;
            pos.1 += i.1;

            if (pos.0 < 0) || (pos.0 >= w) {
                return false;
            }

            if (pos.1 < 0) || (pos.1 >= h) {
                return false;
            }

            if collection[t].contains(&(pos.0 as usize, pos.1 as usize)) {
                return true;
            }
        }
    }
    false
}

pub fn p1_(s: String) {
    let s = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE".to_string();

    // (area, perimeter)
    let mut map: HashMap<char, (i32, i32)> = HashMap::new();
    map.insert('.', (0, 0));

    for row in s.split("\n").map(|x| x.to_string()).collect::<Vec<String>>() {
        let mut prev = '.';
        for i in row.chars() {
            match map.get_mut(&i) {
                Some(c) => {
                    c.0 += 1;
                    if i != prev {
                        c.1 += 1;
                        map.get_mut(&prev).unwrap().1 += 1;
                    }
                },
                None => {
                    map.insert(i, (1, 1));
                    map.get_mut(&prev).unwrap().1 += 1;
                }
            }
            prev = i;
        }
        map.get_mut(&prev).unwrap().1 += 1;
    }

    // rotate and repeat

    let lines = s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>();
    let mut new_one: Vec<String> = (0..lines.len()).map(|_| "".to_string()).collect::<Vec<String>>();
    for row in &lines {
        for (x, item) in row.clone().chars().enumerate() {
            new_one[x].push(item);
        }
    }

    println!("{new_one:?}");

    for row in new_one {
        let mut prev = '.';
        for i in row.chars() {
            match map.get_mut(&i) {
                Some(c) => {
                    c.0 += 1;
                    if i != prev {
                        c.1 += 1;
                        map.get_mut(&prev).unwrap().1 += 1;
                    }
                },
                None => {
                    map.insert(i, (1, 1));
                    map.get_mut(&prev).unwrap().1 += 1;
                }
            }
            prev = i;
        }
        map.get_mut(&prev).unwrap().1 += 1;
    }


    let mut total = 0;
    for i in map {
        if i.0 == '.' {
            continue;
        }
        total += i.1.0 * i.1.1;
    }
    println!("{total}");
}