pub fn p1(s: String) {
    let mut field: Vec<i32> = vec![];
    for (index, i) in s.chars().enumerate() {
        let i = i.to_string().parse::<i32>().unwrap();

        if index % 2 == 1 {
            // is blank
            for _ in 0..i {
                field.push(-1);
            }
        } else {
            // is not blank
            for _ in 0..i {
                field.push(index as i32 / 2);
            }
        }
    }

    loop {
        let far = get_last_non(field.clone());
        let nearest = get_first(field.clone());

        if far.0 == (nearest.0 - 1) {
            break;
        }

        field[far.0] = -1;
        field[nearest.0] = far.1;

        // println!("{}", field.iter().map(|x| if *x == -1 { '.' } else { *x.to_string().chars().collect::<Vec<char>>().first().unwrap() }).collect::<String>());
    }

    let mut result = 0i64;
    for (index, i) in field.iter().enumerate() {
        if *i == -1 {
            break;
        }

        result += index as i64 * *i as i64;
    }

    println!("{result}");

    println!("{}", field.iter().map(|x| if *x == -1 { '.' } else { *x.to_string().chars().collect::<Vec<char>>().first().unwrap() }).collect::<String>());

    // println!("{field:?}");
}

pub fn get_last_non(v: Vec<i32>) -> (usize, i32) {
    let mut v = v;
    v.reverse();
    for (index, i) in v.iter().enumerate() {
        if *i != -1 {
            return ((v.len() - 1) - index, *i);
        }
    }
    panic!();
}

pub fn get_first(v: Vec<i32>) -> (usize, i32) {
    for (index, i) in v.iter().enumerate() {
        if *i == -1 {
            return (index, -1);
        }
    }
    panic!();
}

pub fn p2(s: String) {
    let mut field: Vec<(i32, i32)> = vec![];
    let mut highest_index = 0;
    for (index, i) in s.chars().enumerate() {
        let i = i.to_string().parse::<i32>().unwrap();

        if index % 2 == 1 {
            // is blank
            field.push((-1, i));
        } else {
            // is not blank
            field.push((index as i32 / 2, i));
            highest_index = index as i32 / 2;
        }
    }

    for index in 0..highest_index {
        let index = highest_index - index;

        let matched_element = get_element(&field, index);
        let empty_space = first_empty_space(field.clone(), matched_element.1.1);

        if empty_space.0 == 0 {
            continue;
        }

        if matched_element.0 < empty_space.0 {
            continue;
        }

        let remainder = empty_space.1 - matched_element.1.1;

        field[empty_space.0] = matched_element.1.clone();
        field[matched_element.0] = (-1, matched_element.1.1);

        if remainder != 0 {
            field.insert(empty_space.0 + 1, (-1, remainder));
        }
    }

    let mut new_field: Vec<i32> = vec![];
    for i in field {
        for _ in 0..i.1 {
            new_field.push(i.0);
        }
    }

    let mut result = 0i64;
    for (index, i) in new_field.iter().enumerate() {
        if *i == -1 {
            continue;
        }

        result += index as i64 * *i as i64;
    }

    println!("{result}");
}

fn first_empty_space(v: Vec<(i32, i32)>, size: i32) -> (usize, i32) {
    for (index, i) in v.iter().enumerate() {
        if i.0 != -1 {
            continue;
        }

        if i.1 >= size {
            return (index, i.1);
        }
    }
    (0, 0)
}

fn get_element(v: &Vec<(i32, i32)>, index: i32) -> (usize, (i32, i32)) {
    for (v_i, i) in v.iter().enumerate() {
        if index == i.0 {
            return (v_i, i.clone());
        }
    }
    panic!()
}
