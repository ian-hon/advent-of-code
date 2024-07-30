pub fn p1(s: String) {
    let mut starting_position: (i32, i32) = (0, 0);
    let mut field: Vec<Vec<Pipe>> = vec![];

    for (y, row) in s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>().into_iter().enumerate() {
        let mut pipes: Vec<Pipe> = vec![];
        for (x, c) in row.chars().into_iter().enumerate() {
            pipes.push(parse_char(&c));
            if c == 'S' {
                starting_position = (x as i32, y as i32);
            }
        }
        field.push(pipes);
    }

    // let mut final_steps = 0;

    for first in [(0, 1), (1, 0), (0, -1), (-1 ,0)] {
        let mut current = (starting_position.0 + first.0, starting_position.1 + first.1);
        let mut previous = starting_position.clone();
        let mut steps = 0;

        let mut flag = false;
        loop {
            if (current.0 < 0) || (current.1 < 0) {
                println!("oob");
                break;
            }

            if current == starting_position {
                println!("arrived after {steps} steps");
                flag = true;
                break;
            }

            let result = step(&field[current.1 as usize][current.0 as usize], current, previous);
            steps += 1;

            if result.is_none() {
                println!("no valid steps at : {current:?}");
                break;
            }

            previous = current.clone();
            current = result.unwrap();
        }
        if flag {
            println!("furthest distance is : {}", (steps + 1) / 2);
            break;
        }
    }
}

pub fn p2(s: String) {
//     let s = "FF7FSF7F7F7F7F7F---7
// L|LJ||||||||||||F--J
// FL-7LJLJ||||||LJL-77
// F--JF--7||LJLJIF7FJ-
// L---JF-JLJIIIIFJLJJ7
// |F|F-JF---7IIIL7L|7|
// |FFJF7L7F-JF7IIL---7
// 7-L-JL7||F7|L7F-7F7|
// L.L7LFJ|||||FJL7||LJ
// L7JLJL-JLJLJL--JLJ.L".to_string();
    let mut starting_position: (i32, i32) = (0, 0);
    let mut field: Vec<Vec<Pipe>> = vec![];

    for (y, row) in s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>().into_iter().enumerate() {
        let mut pipes: Vec<Pipe> = vec![];
        for (x, c) in row.chars().into_iter().enumerate() {
            pipes.push(parse_char(&c));
            if c == 'S' {
                starting_position = (x as i32, y as i32);
            }
        }
        field.push(pipes);
    }

    // let mut final_steps = 0;

    let mut loop_elements: Vec<(usize, usize)> = vec![];

    for first in [(0, 1), (1, 0), (0, -1), (-1 ,0)] {
        let mut current = (starting_position.0 + first.0, starting_position.1 + first.1);
        let mut previous = starting_position.clone();
        let mut steps = 0;

        loop_elements = vec![(starting_position.0 as usize, starting_position.1 as usize)];

        let mut flag = false;
        loop {
            if (current.0 < 0) || (current.1 < 0) {
                println!("oob");
                break;
            }

            if current == starting_position {
                println!("arrived after {steps} steps");
                flag = true;
                break;
            }

            let result = step(&field[current.1 as usize][current.0 as usize], current, previous);
            steps += 1;
            loop_elements.push((current.0 as usize, current.1 as usize));

            if result.is_none() {
                println!("no valid steps at : {current:?}");
                break;
            }

            previous = current.clone();
            current = result.unwrap();
        }
        if flag {
            println!("furthest distance is : {}", (steps + 1) / 2);
            break;
        }
    }

    let mut parsed: Vec<String> = vec![];

    for y in 0..field.len() {
        let mut row = "".to_string();
        for x in 0..field.first().unwrap().len() {
            if loop_elements.contains(&(x, y)) {
                row += s.split('\n').collect::<Vec<&str>>()[y].chars().collect::<Vec<char>>()[x].to_string().as_str();
            } else {
                row += '.'.to_string().as_str();
            }
        }
        parsed.push(row);
    }

    println!("{parsed:?}");

    let mut count = 0;

    for row in parsed {
        let mut parsed_row = row.clone().chars().filter(|x| *x != '-').collect::<String>();
        parsed_row = parsed_row.replace("FJ", "|");
        parsed_row = parsed_row.replace("L7", "|");
        // remove -
        // FJ L7 | 

        println!("{}", parsed_row);

        let mut odd = false;
        for c in parsed_row.chars() {
            if c == '|' {
                odd = !odd;
            }

            if !odd {
                continue;
            }

            if c == '.' {
                count += 1;
            }
        }
    }

    println!("count : {count}");

    // the solution uses parity by odd vertical bars to determine the region is inside
}

//   N 
// W   E
//   S

pub fn parse_char(c: &char) -> Pipe {
    match c {
        '-' => Pipe::Horizontal,
        '|' => Pipe::Vertical,
        'L' => Pipe::NE,
        'J' => Pipe::NW,
        '7' => Pipe::SW,
        'F' => Pipe::SE,
        'S' => Pipe::Start,
        _ => Pipe::None
    }
}

#[derive(Debug)]
pub enum Pipe {
    None,
    Start,
    Horizontal, // a-b   b-a
    Vertical,
    NE, // L
    NW, // J
    SW, // 7
    SE  // F
}

fn step(p: &Pipe, current: (i32, i32), previous: (i32, i32)) -> Option<(i32, i32)> {
    // if !is_valid_move(p, current, previous) {
    //     return None;
    // }
    match p {
        Pipe::Horizontal => {
            if previous.0 < current.0 {
                return Some((
                    current.0 + 1,
                    current.1
                ));
            }
            if previous.0 > current.0 {
                return Some((
                    current.0 - 1,
                    current.1
                ));
            }
            None
        },
        Pipe::Vertical => {
            if previous.1 < current.1 {
                return Some((
                    current.0,
                    current.1 + 1
                ));
            }
            if previous.1 > current.1 {
                return Some((
                    current.0,
                    current.1 - 1
                ));
            }
            None
        },
        Pipe::NE => {
            // L
            if current.1 > previous.1 {
                // p
                // c (L)
                return Some((current.0 + 1, current.1));
            }
            if current.0 < previous.0 {
                // c p
                //(L)
                return Some((current.0, current.1 - 1));
            }
            None
        },
        Pipe::NW => {
            // J
            if current.1 > previous.1 {
                // p
                // c (L)
                return Some((current.0 - 1, current.1));
            }
            if current.0 > previous.0 {
                // p c
                //  (J)
                return Some((current.0, current.1 - 1));
            }
            None
        },
        Pipe::SW => {
            // 7
            if current.1 < previous.1 {
                // c (7)
                // p
                return Some((current.0 - 1, current.1));
            }
            if current.0 > previous.0 {
                // p c
                //  (7)
                return Some((current.0, current.1 + 1));
            }
            None
        },
        Pipe::SE => {
            // F
            if current.1 < previous.1 {
                // c (F)
                // p
                return Some((current.0 + 1, current.1));
            }
            if current.0 < previous.0 {
                // c p
                //(F)
                return Some((current.0, current.1 + 1));
            }
            None
        }
        _ => None
    }
}

fn is_valid_move(p: &Pipe, current: (i32, i32), previous: (i32, i32)) -> bool {
    match p {
        _ => false
    }
}
