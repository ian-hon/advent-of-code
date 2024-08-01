pub fn p1(s: String) {
    // only 101475 pairs
    let s = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".to_string();

    let s = expand(s);

    let mut galaxies: Vec<(i32, i32)> = vec![];

    for (y, row) in s.split('\n').enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                galaxies.push((x as i32, y as i32));
            }
        }
    }

    let mut pairs: Vec<((i32, i32), (i32, i32))> = vec![];

    for i in galaxies.clone() {
        for ii in galaxies.clone() {
            if i == ii {
                continue;
            }
            let mut flag = false;
            for p in &pairs {
                if (p.0 == i) && (p.1 == ii) {
                    flag = true;
                }
                if (p.1 == i) && (p.0 == ii) {
                    flag = true;
                }
            }
            if flag {
                continue;
            }

            pairs.push((i, ii));
        }
    }

    let mut length = 0;
    for p in &pairs {
        length += path_length(p.0, p.1);
    }

    println!("{} galaxies; {} pairs", galaxies.len(), pairs.len());
    println!("length : {length}");
}

fn path_length(a: (i32, i32), b: (i32, i32)) -> i32 {
    let mut length = 0;

    let xinc: f32;
    let yinc: f32;
    let mut x: f32;
    let mut y: f32;
    let dx: f32;
    let dy: f32;
    let mut e: f32;
    dx = (b.0 - a.0).abs() as f32;
    dy = (b.1 - a.1).abs() as f32;
    if a.0 < b.0 {
        xinc = 1f32;
    } else {
        xinc = -1f32;
    }
    if a.1 < b.1{
        yinc = 1f32;
    } else {
        yinc = -1f32;
    }
    x = a.0 as f32;
    y = a.1 as f32;
    length += 1;
    if dx >= dy {
       e = (2f32 * dy) - dx;
       while x != b.0 as f32 {
           if e < 0f32 {
               e += 2f32 * dy;
           } else {
               e += 2f32 * (dy - dx);
               y += yinc;
           }
           x += xinc;
           length += 1;
        }
    } else {
        e = (2f32 * dx) - dy;
        while y != b.1 as f32 {
            if e < 0f32 {
                e += 2f32 * dx;
            } else {
                e += 2f32 * (dx - dy);
                x += xinc;
            }
            y += yinc;
            length += 1;
        }
    }

    length
}

fn expand(s: String) -> String {
    let mut result;

    let length = s.split('\n').collect::<Vec<&str>>().first().unwrap().len();
    let query = ".".repeat(length);

    result = s.replace(&query, &format!("{query}\n{query}"));
    result = rotate_string(result);

    let length = result.split('\n').collect::<Vec<&str>>().first().unwrap().len();
    let query = ".".repeat(length);
    result = result.replace(&query, &format!("{query}\n{query}"));

    rotate_string(result)
}

fn rotate_string(s: String) -> String {
    let mut result: Vec<String> = vec![];

    for row in s.split('\n') {
        for (index, c) in row.chars().enumerate() {
            if result.len() <= index {
                result.push("".to_string());
            }
            result[index] += c.to_string().as_str();
        }
    }

    let mut product = "".to_string();

    for r in result {
        product += format!("{r}\n").as_str();
    }

    product
}