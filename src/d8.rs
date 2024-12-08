use std::collections::HashSet;

pub fn p1(s: String) {
    let lines = s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>();

    let width = lines[0].len();
    let height = lines.len();

    let mut antennas: Vec<(i32, i32, char)> = vec![];
    for (y, row) in lines.clone().into_iter().enumerate() {
        for (x, c) in row.clone().chars().enumerate() {
            if c != '.' {
                antennas.push((x as i32, y as i32, c.clone()));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for item in antennas.clone() {
        let similiar = find_simliar(&antennas, item.clone());

        for s in similiar {
            let d_x = s.0 - item.0;
            let d_y = s.1 - item.1;

            let candidates = vec![
                (s.0 + d_x, s.1 + d_y),
                (s.0 - d_x, s.1 - d_y),
                (item.0 + d_x, item.1 + d_y),
                (item.0 - d_x, item.1 - d_y)
            ];

            for c in candidates {
                if (c != (s.0, s.1)) && (c != (item.0, item.1)) {
                    antinodes.insert(c);
                }
            }
        }
    }

    let mut count = 0;
    for c in antinodes.clone() {
        if (c.0 < 0) || (c.0 >= width as i32) {
            continue;
        }

        if (c.1 < 0) || (c.1 >= height as i32) {
            continue;
        }

        count += 1;
    }
    println!("{count}");

    for (y, row) in lines.into_iter().enumerate() {
        let mut result = "".to_string();
        for (x, c) in row.chars().enumerate() {

            if antinodes.contains(&(x as i32, y as i32)) {
                result += "#";
            } else {
                result += c.to_string().as_str();
            }
        }
        println!("{result}");
    }
}

fn find_simliar(a: &Vec<(i32, i32, char)>, c: (i32, i32, char)) -> Vec<(i32, i32, char)> {
    let mut result = vec![];

    for i in a {
        if i.2 == c.2 {
            if (i.0 == c.0) && (i.1 == c.1) {
                continue;
            }
            result.push(i.clone());
        }
    }

    result
}

pub fn p2(s: String) {
    let lines = s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>();

    let width = lines[0].len();
    let height = lines.len();

    let mut antennas: Vec<(i32, i32, char)> = vec![];
    for (y, row) in lines.clone().into_iter().enumerate() {
        for (x, c) in row.clone().chars().enumerate() {
            if c != '.' {
                antennas.push((x as i32, y as i32, c.clone()));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for item in antennas.clone() {
        let similiar = find_simliar(&antennas, item.clone());

        for s in similiar {
            let d_x = s.0 - item.0;
            let d_y = s.1 - item.1;

            let candidates = generate_candidates(width as i32, height as i32, (item.0, item.1), d_x, d_y);

            for c in candidates {
                if (c != (s.0, s.1)) && (c != (item.0, item.1)) {
                    antinodes.insert(c);
                }
            }
        }
    }

    let mut count = 0;
    for c in antinodes.clone() {
        if (c.0 < 0) || (c.0 >= width as i32) {
            continue;
        }

        if (c.1 < 0) || (c.1 >= height as i32) {
            continue;
        }

        let mut flag = false;
        for i in antennas.clone() {
            if (i.0 == c.0) && (i.1 == c.1) {
                flag = true;
                antinodes.remove(&(c.0, c.1));
                break;
            }
        }
        if flag {
            continue;
        }

        count += 1;
    }
    count += antennas.len();
    println!("{count}");

    for (y, row) in lines.into_iter().enumerate() {
        let mut result = "".to_string();
        for (x, c) in row.chars().enumerate() {

            if antinodes.contains(&(x as i32, y as i32)) {
                result += "#";
            } else {
                result += c.to_string().as_str();
            }
        }
        println!("{result}");
    }
}

fn generate_candidates(w: i32, h: i32, start: (i32, i32), dx: i32, dy: i32) -> Vec<(i32, i32)> {
    let mut result = vec![start.clone()];

    let mut pos = start;

    loop {
        pos.0 += dx;
        pos.1 += dy;

        if (pos.0 < 0) || (pos.0 >= w) {
            break;
        }

        if (pos.1 < 0) || (pos.1 >= h) {
            break;
        }

        result.push(pos.clone());
    }

    result
}
