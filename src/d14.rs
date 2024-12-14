use std::collections::HashMap;

pub fn p1(s: String) {
    let s_ = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3".to_string();

    let w = 101;
    let h = 103;

    let mid_w = w / 2;
    let mid_h = h / 2;

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for line in s.split("\n").map(|x| x.to_string()) {
        let pos = line.split(" v=").map(|x| x.to_string()).collect::<Vec<String>>()[0].clone().strip_prefix("p=").unwrap().split(",").map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let vel = line.split(" v=").map(|x| x.to_string()).collect::<Vec<String>>()[1].clone().split(",").map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let final_pos = (
            fix_coords(pos[0] + (vel[0] * 100), w),
            fix_coords(pos[1] + (vel[1] * 100), h)
        );

        match map.get_mut(&final_pos) {
            Some(m) => {
                *m += 1;
            },
            None => {
                map.insert(final_pos, 1);
            }
        }
    }

    // for row in 0..h {
    //     let mut result = "".to_string();
    //     for x in 0..w {
    //         result.push(if (row == mid_h) || (x == mid_w) { ' ' } else {
    //             match map.get(&(x, row)) {
    //                 Some(v) => v.to_string().chars().last().unwrap(),
    //                 None => '.'
    //             }
    //         })
    //     }
    //     println!("{result}");
    // }

    let mut quadrants: Vec<i32> = vec![0, 0, 0, 0];

    for (k, v) in map {
        let x = if k.0 > mid_w { 1 } else if k.0 < mid_w { 0 } else { -1 };
        let y = if k.1 > mid_h { 1 } else if k.1 < mid_h { 0 } else { -1 };

        if (x == -1) || (y == -1) {
            continue;
        }

        quadrants[
            x as usize +
            (y as usize * 2)
        ] += v;
    }

    let mut result = 1;
    for q in quadrants {
        result *= q;
    }
    println!("{result}");
}

pub fn fix_coords(i: i32, b: i32) -> i32 {
    if i.signum() == 1 {
        i % b
    } else {
        ((i % b) + b) % b
    }
}

pub fn fix_coords_2(i: i64, b: i64) -> i64 {
    if i.signum() == 1 {
        i % b
    } else {
        ((i % b) + b) % b
    }
}

pub fn p2(s: String) {
    let s_ = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3".to_string();

    let w = 101;
    let h = 103;

    let mid_w = w / 2;
    let mid_h = h / 2;

    for step_count in 0..100_000 {
        let step_count = step_count;
        println!("step : {step_count}");

        let mut map: HashMap<(i64, i64), i64> = HashMap::new();

        for line in s.split("\n").map(|x| x.to_string()) {
            let pos = line.split(" v=").map(|x| x.to_string()).collect::<Vec<String>>()[0].clone().strip_prefix("p=").unwrap().split(",").map(|x| x.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>();
            let vel = line.split(" v=").map(|x| x.to_string()).collect::<Vec<String>>()[1].clone().split(",").map(|x| x.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>();

            let final_pos = (
                fix_coords_2(pos[0] + (vel[0] * step_count), w),
                fix_coords_2(pos[1] + (vel[1] * step_count), h)
            );

            match map.get_mut(&final_pos) {
                Some(m) => {
                    *m += 1;
                },
                None => {
                    map.insert(final_pos, 1);
                }
            }
        }

        let mut flag = false;
        for row in 0..h {
            let mut result = "".to_string();
            for x in 0..w {
                result.push(match map.get(&(x, row)) {
                    Some(v) => '#',
                    None => ' '
                })
            }
            if result.contains("#############") {
                flag = true;
            }
        }

        if flag {
            for row in 0..h {
                let mut result = "".to_string();
                for x in 0..w {
                    result.push(match map.get(&(x, row)) {
                        Some(v) => '#',
                        None => ' '
                    })
                }
                println!("{result}");
            }
            println!("{step_count} seconds");
            break;
        }

        // 958
    }
}