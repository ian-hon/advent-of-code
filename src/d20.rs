// use std::collections::HashSet;

// pub fn p1(s: String) {
//     let s = "###############
// #...#...#.....#
// #.#.#.#.#.###.#
// #S#...#.#.#...#
// #######.#.#.###
// #######.#.#...#
// #######.#.###.#
// ###..E#...#...#
// ###.#######.###
// #...###...#...#
// #.#####.#.###.#
// #.#...#.#.#...#
// #.#.#.#.#.#.###
// #...#...#...###
// ###############".to_string();
//     let mut field: Vec<Vec<i32>> = vec![];

//     let mut position = (0, 0);
//     let mut target = (0, 0);

//     for (y, row) in s.split("\n").map(|x| x.to_string()).enumerate() {
//         field.push(vec![]);
//         for (x, c) in row.chars().enumerate() {
//             field[y].push(if c == '#' { 1 } else { 0 });
//             if c == 'S' {
//                 position = (x, y);
//             }
//             if c == 'E' {
//                 target = (x, y);
//             }
//         }
//     }

//     let h = field.len() as i32;
//     let w = field[0].len() as i32;

//     let mut nodes: Vec<((usize, usize), Vec<(usize, usize)>, i32)> = vec![];
//     nodes.push((position, vec![position], 0));

//     let mut traversed: HashSet<(usize, usize)> = HashSet::new();
//     traversed.insert(position);

//     let mut routes = vec![];

//     loop {
//         println!("{}", routes.len());

//         let mut next = vec![];
//         for n in nodes {
//             let mut n = n;

//             // bypass
//             // -1 -> used already
//             // 0 -> never used
//             // 1 -> first second
//             // 2 -> second second
//             if n.2 >= 1 {
//                 // bypass mode activated
//                 n.2 += 1;
//                 if n.2 >= 3 {
//                     n.2 = -1;
//                 }
//             }

//             for neighbour in get_neighbours(n.0.clone(), &field, &traversed, w, h, n.2 >= 1) {
//                 let mut n = n.clone();
//                 n.0 = neighbour;
//                 n.1.push(neighbour);

//                 if neighbour == target {
//                     routes.push(n.1.clone());
//                     continue;
//                 } else {
//                     traversed.insert(neighbour);
//                 }

//                 next.push(n.clone());
//             }

//             if n.2 == 0 {
//                 n.2 = 1;
//                 for neighbour in get_neighbours(n.0.clone(), &field, &traversed, w, h, n.2 >= 1) {
//                     let mut n = n.clone();
//                     n.0 = neighbour;
//                     n.1.push(neighbour);
    
//                     if neighbour == target {
//                         routes.push(n.1.clone());
//                         continue;
//                     } else {
//                         traversed.insert(neighbour);
//                     }
    
//                     next.push(n.clone());
//                 }
//             }
//         }
//         nodes = next;
//     }
// }

// pub fn traverse() -> bool {
//     false
// }

// pub fn get_neighbours(p: (usize, usize), f: &Vec<Vec<i32>>, t: &HashSet<(usize, usize)>, w: i32, h: i32, bypass: bool) -> Vec<(usize, usize)> {
//     let p = (p.0 as i32, p.1 as i32);
//     let mut result = vec![];
//     for c in vec![
//         (p.0 + 1, p.1),
//         (p.0 - 1, p.1),
//         (p.0, p.1 + 1),
//         (p.0, p.1 - 1),
//     ] {
//         if (c.0 <= 0) || (c.0 >= w) {
//             continue;
//         }
//         if (c.1 <= 0) || (c.1 >= h) {
//             continue;
//         }

//         let c = (c.0 as usize, c.1 as usize);

//         if t.contains(&c) {
//             continue;
//         }

//         if bypass || (f[c.1][c.0] == 0) {
//             result.push(c);
//         }
//     }
//     result
// }

use std::collections::{HashMap, HashSet};

pub fn p1(s: String) {
    let _s = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############".to_string();
let _s = "#####
#S..#
#...#
#...#
#..E#
#####".to_string();
    let mut field: Vec<Vec<bool>> = vec![];
    // true represents non-wall

    let mut position = (0, 0);
    let mut target = (0, 0);

    for (y, row) in s.split("\n").map(|x| x.to_string()).enumerate() {
        field.push(vec![]);
        for (x, c) in row.chars().enumerate() {
            field[y].push(c != '#');
            if c == 'S' {
                position = (x, y);
            }
            if c == 'E' {
                target = (x, y);
            }
        }
    }

    let h = field.len() as i32;
    let w = field[0].len() as i32;

    let (original_timing, original_route, pos_map) = get_timing(position, target, &field, h, w);
    println!("original : {original_timing}");

    let mut removeable = vec![];

    for r in &original_route {
        for n in get_neighbours(r.clone(), &field, &HashSet::new(), h, w, true) {
            if !original_route.contains(&n) {
                removeable.push(n);
            }
        }
    }

    let removeable: HashSet<(usize, usize)> = HashSet::from_iter(removeable);

    let mut result_count = 0;
    for r in removeable {
        let mut f = field.clone();
        f[r.1][r.0] = true;

        let neighbour_values = get_neighbours(r, &f, &HashSet::new(), h, w, false).iter().map(|x| pos_map.get(x).unwrap().clone()).collect::<Vec<usize>>();

        let highest = neighbour_values.clone().iter().max().unwrap().clone();
        let lowest = neighbour_values.iter().min().unwrap().clone();

        let d = highest as i32 - lowest as i32 - 2;

        if d >= 100 {
            result_count += 1;
        }
    }
    println!("{result_count}");



    // display_route(h, w, position, removeable.clone(), &field);

    // let mut timings = vec![];
    // let mut result_count = 0;
    // for (index, r) in removeable.iter().enumerate() {
    //     println!("{index}/{}", removeable.len());

    //     let mut f = field.clone();
    //     f[r.1][r.0] = true;

    //     let (t, _) = get_timing(position, target, &f, h, w);
    //     if (original_timing - t) >= 100 {
    //         result_count += 1;
    //     }
    //     // timings.push(original_timing - t);
    // }
    // println!("{result_count:?}");


    // let mut routes: Vec<((usize, usize), Vec<(usize, usize)>, HashSet<(usize, usize)>, usize, Option<(usize, usize)>, Option<(usize, usize)>)> = vec![(position, vec![position], HashSet::new(), 1, None, None)];
    // let mut cheats: HashMap<((usize, usize), (usize, usize)), i32> = HashMap::new();

    // loop {
    //     println!("{} cheats found, {} routes", cheats.len(), routes.len());

    //     let mut new_routes = vec![];

    //     for r in routes {
    //         // add all neighbours
    //         // add all neighbours (cheat)
    //         if (r.3 == 1) || (r.3 == 0) {
    //             // add non-cheat children if amount == 2
    //             for n in get_neighbours(r.0, &field, &r.2, h, w, false) {
    //                 let mut new_map = r.2.clone();
    //                 new_map.insert(n);

    //                 let mut new_history = r.1.clone();
    //                 new_history.push(n);

    //                 if n == target {
    //                     let t = original_timing as i32 - new_history.len() as i32;
    //                     if t <= 0 {
    //                         continue;
    //                     }
    //                     let start = match r.4 {
    //                         Some(i) => i,
    //                         None => n
    //                     };
    //                     let end = match r.5 {
    //                         Some(i) => i,
    //                         None => n
    //                     };

    //                     match cheats.get_mut(&(start, end)) {
    //                         Some(c) => {
    //                             *c = (c.clone()).max(t);
    //                         },
    //                         None => {
    //                             cheats.insert((start, end), t);
    //                         }
    //                     }

    //                     // cheats.insert()
    //                     // println!("t : {}", t);
    //                     // println!("{:?}\t{:?}", start, end);
    //                     // display_route(h, w, position, new_map, &field);
    //                     continue;
    //                     // complete_route = (new_history, new_map);
    //                     // break 'outermost;
    //                 } else {
    //                     new_routes.push((n, new_history, new_map, r.3, r.4, r.5));
    //                 }
    //             }
    //         }
    //         if r.3 != 0 {
    //             let current = r.3 - 1;
    //             for n in get_neighbours(r.0, &field, &r.2, h, w, true) {
    //                 let mut new_map = r.2.clone();
    //                 new_map.insert(n);

    //                 let mut new_history = r.1.clone();
    //                 new_history.push(n);

    //                 if n == target {
    //                     let t = original_timing as i32 - new_history.len() as i32;
    //                     if t <= 0 {
    //                         continue;
    //                     }
    //                     let start = match r.4 {
    //                         Some(i) => i,
    //                         None => n
    //                     };
    //                     let end = match r.5 {
    //                         Some(i) => i,
    //                         None => n
    //                     };

    //                     match cheats.get_mut(&(start, end)) {
    //                         Some(c) => {
    //                             *c = (c.clone()).max(t);
    //                         },
    //                         None => {
    //                             cheats.insert((start, end), t);
    //                         }
    //                     }

    //                     // println!("t : {}", t);
    //                     // println!("{:?}\t{:?}", start, end);
    //                     // display_route(h, w, position, new_map, &field);
    //                     continue;
    //                     // complete_route = (new_history, new_map);
    //                     // break 'outermost;
    //                 } else {
    //                     let nh = new_history.clone();
    //                     new_routes.push((n, new_history, new_map, current, if current == 1 { Some(nh[nh.len() - 2]) } else { r.4 }, if current == 0 { Some(n) } else { r.5 }));
    //                 }
    //             }
    //         }
    //     }

    //     if new_routes.is_empty() {
    //         break;
    //     }

    //     routes = new_routes;
    // }

    // let mut result: HashMap<i32, i32> = HashMap::new();
    // for (_, v) in cheats {
    //     match result.get_mut(&v) {
    //         Some(r) => {
    //             *r += 1
    //         },
    //         None => {
    //             result.insert(v, 1);
    //         }
    //     }
    // }

    // let mut final_count = 0;
    // for i in result {
    //     if i.0 >= 100 {
    //         final_count += i.1;
    //     }
    // }

    // println!("{final_count}");

    // println!("{}", complete_route.0.len());

    // for y in 0..h {
    //     let mut result = "".to_string();
    //     for x in 0..w {
    //         result += if (x as usize, y as usize) == position { "S" } else { if complete_route.1.contains(&(x as usize, y as usize)) { "#" } else { if field[y as usize][x as usize] { " " } else { "." } }}
    //     }
    //     println!("{result}");
    // }
}

pub fn get_timing(position: (usize, usize), target: (usize, usize), field: &Vec<Vec<bool>>, h: i32, w: i32) -> (usize, Vec<(usize, usize)>, HashMap<(usize, usize), usize>) {
    let mut routes: Vec<((usize, usize), Vec<(usize, usize)>, HashSet<(usize, usize)>)> = vec![(position, vec![position], HashSet::from_iter(vec![position]))];

    let mut pos_map: HashMap<(usize, usize), usize> = HashMap::from_iter(vec![(position, 0)]);

    loop {
        let mut new_routes = vec![];

        for r in routes {
            for n in get_neighbours(r.0, &field, &r.2, h, w, false) {
                let mut new_map = r.2.clone();
                new_map.insert(n);

                let mut new_history = r.1.clone();
                new_history.push(n);

                pos_map.insert(n, pos_map.len());

                if n == target {
                    return (new_history.len(), new_history, pos_map);
                } else {
                    new_routes.push((n, new_history, new_map));
                }
            }
        }

        routes = new_routes;
    }
}

pub fn display_route(h: i32, w: i32, position: (usize, usize), r: HashSet<(usize, usize)>, field: &Vec<Vec<bool>>) {
    for y in 0..h {
        let mut result = "".to_string();
        for x in 0..w {
            result += if (x as usize, y as usize) == position { "S" } else { if r.contains(&(x as usize, y as usize)) { "#" } else { if field[y as usize][x as usize] { " " } else { "." } }}
        }
        println!("{result}");
    }
}

pub fn get_neighbours(p: (usize, usize), field: &Vec<Vec<bool>>, map: &HashSet<(usize, usize)>, h: i32, w: i32, ignore_collision: bool) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let p = (p.0 as i32, p.1 as i32);

    for c in [
        (p.0 + 1, p.1),
        (p.0 - 1, p.1),
        (p.0, p.1 + 1),
        (p.0, p.1 - 1),
    ] {
        if (c.0 < 0) || (c.1 < 0) || (c.0 >= w) || (c.1 >= h) {
            continue;
        }

        let c = (c.0 as usize, c.1 as usize);

        if map.contains(&c) {
            continue;
        }

        if (!ignore_collision) && (!field[c.1][c.0]) {
            continue;
        }

        result.push(c);
    }

    result
}
