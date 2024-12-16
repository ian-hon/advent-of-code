// use std::collections::HashSet;

// pub fn p1(s: String) {
//     let s = "###############
// #.......#....E#
// #.#.###.#.###.#
// #.....#.#...#.#
// #.###.#####.#.#
// #.#.#.......#.#
// #.#.#####.###.#
// #...........#.#
// ###.#.#####.#.#
// #...#.....#.#.#
// #.#.#.###.#.#.#
// #.....#...#.#.#
// #.###.#.#.#.#.#
// #S..#.....#...#
// ###############".to_string();

//     let s_ = "#################
// #...#...#...#..E#
// #.#.#.#.#.#.#.#.#
// #.#.#.#...#...#.#
// #.#.#.#.###.#.#.#
// #...#.#.#.....#.#
// #.#.#.#.#.#####.#
// #.#...#.#.#.....#
// #.#.#####.#.###.#
// #.#.#.......#...#
// #.#.###.#####.###
// #.#.#...#.....#.#
// #.#.#.#####.###.#
// #.#.#.........#.#
// #.#.#.#########.#
// #S#.............#
// #################".to_string();

//     let mut nodes: Vec<((usize, usize), Vec<(usize, usize)>)> = vec![];
//     let mut map: HashSet<(usize, usize)> = HashSet::new();
//     let mut target = (0, 0);

//     let mut field: Vec<Vec<i32>> = vec![];
//     for (y, row) in s.split("\n").map(|x| x.to_string()).into_iter().enumerate() {
//         field.push(vec![]);
//         for (x, e) in row.chars().enumerate() {
//             field[y].push(match e {
//                 '#' => 1,
//                 _ => 0
//             });

//             if e == 'S' {
//                 nodes.push(((x, y), vec![]));
//             }

//             if e == 'E' {
//                 target = (x, y);
//             }
//         }
//     }



//     let mut final_paths: Vec<Vec<(usize, usize)>> = vec![];

//     let mut steps = 0;
//     loop {
//         steps += 1;
//         let mut next: Vec<((usize, usize), Vec<(usize, usize)>)> = vec![];
//         let mut flag = false;
//         for node in nodes.clone() {
//             let mut past = node.1.clone();

//             for neighbour in get_neighbours(node.0, &field, *past.iter().last().unwrap()) {
//                 if neighbour == target {
//                     let mut p = past.clone();
//                     p.push(node.0);
//                     p.push(neighbour);

//                     final_paths.push(p);

//                     println!("paths found : {}", final_paths.len());

//                     if final_paths.len() >= 10 {
//                         flag = true;
//                         break;
//                     }
//                 }

//                 // map.insert(neighbour);
//                 let mut p = past.clone();
//                 p.push(node.0);
//                 next.push((neighbour, p));
//             }
//             if flag {
//                 break;
//             }
//         }
//         if flag {
//             break;
//         }
//         nodes = next;
//     }

//     for final_path in final_paths {
//         for (y, row) in s.split("\n").map(|x| x.to_string()).into_iter().enumerate() {
//             let mut result = "".to_string();
//             for (x, e) in row.chars().enumerate() {
//                 result.push(if final_path.contains(&(x, y)) { '*' } else { match e {
//                     '#' => '.',
//                     _ => ' '
//                 } });
//             }
//             println!("{result}");
//         }
//     }

//     println!("steps : {steps}");
// }

// pub fn get_neighbours(p: (usize, usize), f: &Vec<Vec<i32>>, prev: (usize, usize)) -> Vec<(usize, usize)> {
//     let mut result = vec![];
//     for c in vec![
//         (0, -1),
//         (1, 0),
//         (0, 1),
//         (-1, 0)
//     ] {
//         let candidate = (
//             (p.0 as i32 + c.0) as usize,
//             (p.1 as i32 + c.1) as usize,
//         );

//         if candidate == prev {
//             continue;
//         }

//         if f[candidate.1][candidate.0] == 0 {
//             result.push(candidate);
//         }
//     }
//     result
// }

use std::{collections::HashSet, i32};

pub fn p1(s: String) {
    let s_ = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############".to_string();

    let s_ = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################".to_string();

    let mut nodes: Vec<((usize, usize, usize), Vec<(usize, usize, usize)>)> = vec![];
    let mut map: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut target = (0, 0);

    let mut field: Vec<Vec<i32>> = vec![];
    for (y, row) in s.split("\n").map(|x| x.to_string()).into_iter().enumerate() {
        field.push(vec![]);
        for (x, e) in row.chars().enumerate() {
            field[y].push(match e {
                '#' => 1,
                _ => 0
            });

            if e == 'S' {
                nodes.push(((1, x, y), vec![]));
            }

            if e == 'E' {
                target = (x, y);
            }
        }
    }

    let mut lowest_score = 101436;
    let mut final_paths: Vec<Vec<(usize, usize, usize)>> = vec![];

    let mut map: HashSet<(usize, usize, usize)> = HashSet::new();

    let mut steps = 0;
    loop {
        steps += 1;
        println!("{steps} : {}", nodes.len());

        if nodes.is_empty() {
            break;
        }

        let mut next: Vec<((usize, usize, usize), Vec<(usize, usize, usize)>)> = vec![];
        for node in nodes.clone() {
            let neighbours = get_neighbours(node.0, &field);
            if neighbours.is_empty() {
                continue;
            }
            for neighbour in neighbours {
                // if map.contains(&neighbour) {
                //     continue;
                // }
                // map.insert(neighbour.clone());
                
                // println!("neighbour : {neighbour:?}");
                let mut p = node.1.clone();
                p.push(node.0);

                if (neighbour.1 == target.0) && (neighbour.2 == target.1) {
                    p.push(neighbour);
                    final_paths.push(p.clone());

                    lowest_score = lowest_score.min(get_score(p));
                    println!("{} : {lowest_score}", final_paths.len());
                    continue;
                }

                if get_score(p.clone()) > lowest_score {
                    continue;
                }

                next.push((neighbour, p));
            }
        }
        nodes = next;
    }

    for final_path in final_paths {
        if lowest_score != get_score(final_path.clone()) {
            continue;
        }

        for (y, row) in s.split("\n").map(|x| x.to_string()).into_iter().enumerate() {
            let mut result = "".to_string();
            for (x, e) in row.chars().enumerate() {
                result.push(
                    if final_path.clone().iter().filter(|item| (item.1 == x) && (item.2 == y)).count() != 0 {
                        '*'
                    } else {
                        match e {
                            '#' => '.',
                            _ => ' '
                        }
                    }
                );
            }
            println!("{result}");
        }
    }

    println!("steps : {steps}\tlowest : {lowest_score}");

    // 101436 too high
}

pub fn get_neighbours(p: (usize, usize, usize), f: &Vec<Vec<i32>>) -> Vec<(usize, usize, usize)> {
    let mut result = vec![];
    for (index, c) in vec![
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 0),
    ].iter().enumerate() {
        if p.0 == index {
            // if candidate is up, and current is down, continue
            // prevent from going backwards
            continue;
        }

        let candidate = (
            (p.1 as i32 + c.0) as usize,
            (p.2 as i32 + c.1) as usize,
        );

        if f[candidate.1][candidate.0] == 0 {
            result.push((match index {
                0 => 2,
                1 => 3,
                2 => 0,
                3 => 1,
                _ => panic!()
            }, candidate.0, candidate.1));
        }
    }
    result
}

pub fn get_score(f: Vec<(usize, usize, usize)>) -> i32 {
    let mut forward_steps = -1;
    let mut turns = 0;
    let mut prev = 1;
    for i in f {
        if i.0 != prev {
            turns += 1;
        }
        forward_steps += 1;
        prev = i.0;
    }

    (turns * 1000) + forward_steps
}
