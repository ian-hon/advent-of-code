pub fn p1(s: String) {
    let s = expand(s);

    let mut galaxies: Vec<(i64, i64)> = vec![];

    for (y, row) in s.split('\n').enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                galaxies.push((x as i64, y as i64));
            }
        }
    }

    println!("{} galaxies", galaxies.len());

    let mut pairs: Vec<((i64, i64), (i64, i64))> = vec![];

    for i in 0..galaxies.len() {
        for ii in (i + 1)..galaxies.len() {
            pairs.push((galaxies[i].clone(), galaxies[ii].clone()));
        }
    }

    println!("{} pairs", pairs.len());

    let mut length = 0;
    for p in pairs.clone() {
        length += path_length(p.0, p.1);
    }

    println!("length : {length}");
}

pub fn p2(s: String) {
    let empty_spaces = expand_million(s.clone());

    let expansion_amount = 1_000_000;

    let mut galaxies: Vec<(i64, i64)> = vec![];
    let mut y = 0;
    for (y_index, row) in s.split('\n').enumerate() {
        let mut x = 0;
        for (x_index, c) in row.chars().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
            }
            if empty_spaces.0.contains(&x_index) {
                x += expansion_amount;
            } else {
                x += 1;
            }
        }
        if empty_spaces.1.contains(&y_index) {
            y += expansion_amount;
        } else {
            y += 1;
        }
    }

    println!("{} galaxies", galaxies.len());

    let mut pairs: Vec<((i64, i64), (i64, i64))> = vec![];

    for i in 0..galaxies.len() {
        for ii in (i + 1)..galaxies.len() {
            pairs.push((galaxies[i].clone(), galaxies[ii].clone()));
        }
    }

    println!("{} pairs", pairs.len());

    let mut length = 0;
    for p in pairs.clone() {
        length += path_length(p.0, p.1);
    }

    println!("length : {length}");
}

fn path_length(a: (i64, i64), b: (i64, i64)) -> i64 {
    let width = (a.0 - b.0).abs();
    let height = (a.1 - b.1).abs();

    // might not need bresenham's line algorithm

    width + height
}

fn expand_million(s: String) -> (Vec<usize>, Vec<usize>) {
    let mut result = (vec![], vec![]);

    for (index, row) in s.split('\n').enumerate() {
        let length = row.len();
        if row == ".".repeat(length) {
            result.1.push(index);
        }
    }

    let s = rotate_string(s).strip_suffix('\n').unwrap().to_string();

    for (index, row) in s.split('\n').enumerate() {
        let length = row.len();
        if row == ".".repeat(length) {
            result.0.push(index);
        }
    }

    result
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