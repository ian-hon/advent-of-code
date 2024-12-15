pub fn p1(s: String) {
    let s_ = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<".to_string();
    let mut instructions_raw = vec![];
    for i in s.split("\n\n").last().unwrap().to_string().split("\n").map(|x| x.to_string().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>() {
        for c in i {
            instructions_raw.push(c);
        }
    }

    let mut instructions: Vec<(i32, i32)> = vec![];
    for i in instructions_raw {
        instructions.push(match i {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => panic!()
        });
    }

    let mut position = (0, 0);

    let mut field: Vec<Vec<i32>> = vec![];
    for (y, i) in s.split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>()[0].split("\n").map(|x| x.to_string()).collect::<Vec<String>>().iter().enumerate() {
        field.push(vec![]);
        for (x, c) in i.chars().enumerate() {
            // instructions_raw.push(c);
            field[y].push(match c {
                '#' => 2,
                'O' => 1,
                _ => 0
            });

            if c == '@' {
                position = (x as i32, y as i32);
            }
        }
    }

    for instruction in instructions {
        let next_pos = (
            position.0 + instruction.0,
            position.1 + instruction.1
        );
        let r = field[next_pos.1 as usize][next_pos.0 as usize];
        match r {
            0 => {
                position = next_pos;
            },
            1 => {
                if push(&mut field, position, instruction) {
                    position = next_pos;
                }
            },
            _ => {}
        }
    }

    let mut score = 0;
    for (y, row) in field.iter().enumerate() {
        let mut result = "".to_string();
        for (x, i) in row.iter().enumerate() {
            result.push(if (y == position.1 as usize) && (x == position.0 as usize) { '@' } else {
                match i {
                    0 => '.',
                    1 => 'O',
                    2 => '#',
                    _ => panic!()
                }
            });

            if *i == 1 {
                score += (100 * y) + x;
            }
        }
        println!("{result}");
    }

    println!("{score}");
}

pub fn push(field: &mut Vec<Vec<i32>>, pos: (i32, i32), d: (i32, i32)) -> bool {
    // pos is the current pos of @
    // true if able to push
    let mut start = pos;
    let mut pos = pos;
    loop {
        pos.0 += d.0;
        pos.1 += d.1;

        let next = field[pos.1 as usize][pos.0 as usize];

        if next == 2 {
            return false;
        }

        if next == 0 {
            field[(start.1 + d.1) as usize][(start.0 + d.0) as usize] = 0;
            field[pos.1 as usize][pos.0 as usize] = 1;

            return true;
        }
    }

    // @OOO
}

pub fn p2(s: String) {
    let s = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^".to_string();
    let mut instructions: Vec<(i32, i32)> = vec![];
    for i in s.split("\n\n").last().unwrap().to_string().split("\n").map(|x| x.to_string().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>() {
        for c in i {
            instructions.push(match c {
                '^' => (0, -1),
                '>' => (1, 0),
                'v' => (0, 1),
                '<' => (-1, 0),
                _ => panic!()
            });
        }
    }

    let mut position = (0, 0);
    let mut field: Vec<Vec<i32>> = vec![];
    for (y, i) in s.split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>()[0].split("\n").map(|x| x.to_string()).collect::<Vec<String>>().iter().enumerate() {
        field.push(vec![]);
        for (x, c) in i.chars().enumerate() {
            for item in match c {
                '#' => vec![3, 3],
                'O' => vec![1, 2],
                _ => vec![0, 0]
            } {
                field[y].push(item);
            }

            if c == '@' {
                position = ((x as i32) * 2, y as i32);
            }
        }
    }

    for instruction in instructions {
        let next_pos = (
            position.0 + instruction.0,
            position.1 + instruction.1
        );
        let r = field[next_pos.1 as usize][next_pos.0 as usize];
        match r {
            0 => {
                position = next_pos;
            },
            1 => {
                // if push(&mut field, position, instruction) {
                //     position = next_pos;
                // }
            },
            _ => {}
        }

        for (y, row) in field.iter().enumerate() {
            let mut result = "".to_string();
            for (x, i) in row.iter().enumerate() {
                result.push(if (y == position.1 as usize) && (x == position.0 as usize) { '@' } else {
                    match i {
                        0 => '.',
                        1 => '[',
                        2 => ']',
                        3 => '#',
                        _ => panic!()
                    }
                });
            }
            println!("{result}");
        }
    }

    let mut score = 0;
    for (y, row) in field.iter().enumerate() {
        let mut result = "".to_string();
        for (x, i) in row.iter().enumerate() {
            result.push(if (y == position.1 as usize) && (x == position.0 as usize) { '@' } else {
                match i {
                    0 => '.',
                    1 => '[',
                    2 => ']',
                    3 => '#',
                    _ => panic!()
                }
            });

            if *i == 1 {
                score += (100 * y) + x;
            }
        }
        println!("{result}");
    }

    println!("{score}");
}

pub fn push2(field: &mut Vec<Vec<i32>>, pos: (i32, i32), d: (i32, i32)) -> bool {
    // pos is the current pos of @
    // true if able to push

    let mut pushables: Vec<(i32, (i32, i32))> = vec![];
    let mut pos = pos;
    loop {
        pos.0 += d.0;
        pos.1 += d.1;

        let next = field[pos.1 as usize][pos.0 as usize];

        if next == 2 {
            return false;
        }

        if next == 0 {
            /*
            consider the following
            [][][][][]
             [][][][]
              [][][]
               [][]
                []
                @
             */
            /*
            or
            [][][][][]
             [][][][]
              [][]##
               [][]
                []
                @
             */
            if d.0 == 0 {
                // vertical
                let mut containers: Vec<((i32, i32), (i32, i32))> = vec![];
                // 1: [, 2: ]
                for p in pushables {
                    if p.0 == 1 {
                        // containers.push((
                        //     (),
                            
                        // ))
                        // TODO: finish
                    }
                }
            } else {
                // horizontal
            }

            return true;
        }

        pushables.push((next, pos.clone()));
    }

    // @OOO
}
