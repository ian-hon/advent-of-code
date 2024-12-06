pub fn p1(s: String) {
    let s_ = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...".to_string();

    let height = s.split('\n').count() as i32;
    let width = s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>()[0].len() as i32;

    let mut field: Vec<Vec<i32>> = vec![];
    let mut position: (usize, usize) = (0, 0);

    let lines = s.split('\n').into_iter().map(|x| x.to_string()).collect::<Vec<String>>();

    for (row_index, row) in lines.iter().enumerate() {
        field.push(vec![]);
        for (index, c) in row.clone().chars().enumerate() {
            if c == '^' {
                position = (index, row_index);
                field[row_index].push(2);
                continue;
            }
            field[row_index].push(if c == '#' { 1 } else { 0 });
        }
    }

    // field[position.1][position.0] = 2;

    // let mut steps = 0;
    let mut direction = 0; // up, right, down, left
    loop {
        let mut next_pos = match direction {
            0 => (position.0 as i32, position.1 as i32 - 1),
            1 => (position.0 as i32 + 1, position.1 as i32),
            2 => (position.0 as i32, position.1 as i32 + 1),
            3 => (position.0 as i32 - 1, position.1 as i32),
            _ => (0, 0)
        };

        // check if in map
        if (next_pos.0 < 0) || (next_pos.0 >= width) {
            break;
        }
        if (next_pos.1 < 0) || (next_pos.1 >= height) {
            break;
        }

        if field[next_pos.1 as usize][next_pos.0 as usize] == 1 {
            direction += 1;
            if direction == 4 {
                direction = 0;
            }
            continue;
        }

        let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

        field[next_pos.1][next_pos.0] = 2;
        position = next_pos;
    }

    let mut sum = 0;
    for row in field {
        for i in row {
            if i == 2 {
                sum += 1;
            }
        }
    }
    println!("{sum}");

    // 4931 not the right answer
}

pub fn p2(s: String) {
    let s_ = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...".to_string();

    let s_= ".....
.#...
....#
.^...
.....
...#.".to_string();

    let height = s.split('\n').count() as i32;
    let width = s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>()[0].len() as i32;

    let mut field: Vec<Vec<(i32, i32)>> = vec![];
    let mut position: (usize, usize) = (0, 0);

    let lines = s.split('\n').into_iter().map(|x| x.to_string()).collect::<Vec<String>>();

    for (row_index, row) in lines.iter().enumerate() {
        field.push(vec![]);
        for (index, c) in row.clone().chars().enumerate() {
            if c == '^' {
                position = (index, row_index);
                field[row_index].push((2, 1));
                continue;
            }
            field[row_index].push(if c == '#' { (1, 0) } else { (0, 0) });
        }
    }

    // field[position.1][position.0] = 2;

    // let mut steps = 0;
    let mut direction = 0; // up, right, down, left
    let mut count = 0;
    loop {
        let mut next_pos = match direction {
            0 => (position.0 as i32, position.1 as i32 - 1),
            1 => (position.0 as i32 + 1, position.1 as i32),
            2 => (position.0 as i32, position.1 as i32 + 1),
            3 => (position.0 as i32 - 1, position.1 as i32),
            _ => (0, 0)
        };

        // check if in map
        if (next_pos.0 < 0) || (next_pos.0 >= width) {
            break;
        }
        if (next_pos.1 < 0) || (next_pos.1 >= height) {
            break;
        }

        let next_direction = if direction == 3 { 0 } else { direction + 1 };

        if field[next_pos.1 as usize][next_pos.0 as usize].0 == 1 {
            mark_directions(&mut field, position, direction, width, height);
            direction = next_direction;
            continue;
        }

        let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

        if ((field[next_pos.1][next_pos.0].1 >> next_direction) & 1) == 1 {
            count += 1;
        }

        field[next_pos.1][next_pos.0].0 = 2;
        field[next_pos.1][next_pos.0].1 |= 1 << direction;
        position = next_pos;
    }

    let mut sum = 0;
    for row in field {
        let mut r = "".to_string();
        for i in row {
            // if i.1 != 0 {
            //     if (i.1 & 1) + ((i.1 >> 2) & 1) == 0 {
            //         r += "-";
            //     } else {
            //         if ((i.1 >> 1) & 1) + ((i.1 >> 3) & 1) == 0 {
            //             r += "|";
            //         } else {
            //             r += "+";
            //         }
            //     }
            //     // r += format!("{:x}", i.1).as_str();
            // } else {
            //     if i.0 == 1 {
            //         r += "#";
            //     } else {
            //         r += " ";
            //     }
            // }
            
            // if i.0 == 2 {
            //     if (i.1 & 1) + ((i.1 >> 2) & 1) == 0 {
            //         r += "-";
            //     } else {
            //         if ((i.1 >> 1) & 1) + ((i.1 >> 3) & 1) == 0 {
            //             r += "|";
            //         } else {
            //             r += "+";
            //         }
            //     }
            //     // r += format!("{:x}", i.1).as_str();
            // } else {
            //     if i.0 == 1 {
            //         r += "#";
            //     } else {
            //         r += " ";
            //     }
            // }
            if i.0 == 2 {
                sum += 1;
            }
        }
        println!("{r}");
    }
    println!("{sum}\t{count}");

}

pub fn mark_directions(field: &mut Vec<Vec<(i32, i32)>>, pos: (usize, usize), direction: i32, width: i32, height: i32) {
    let increment = vec![
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 0)
    ][direction as usize];
    let mut pos = pos;
    loop {
        let next_pos = (pos.0 as i32 + increment.0, pos.1 as i32 + increment.1);

        if (next_pos.0 < 0) || (next_pos.0 >= width) {
            return;
        }
        if (next_pos.1 < 0) || (next_pos.1 >= height) {
            return;
        }

        let next_pos = (next_pos.0 as usize, next_pos.1 as usize);

        if field[next_pos.1][next_pos.0].0 == 1 {
            return;
        }

        pos = next_pos;
        field[next_pos.1][next_pos.0].1 |= 1 << direction;
    }
}