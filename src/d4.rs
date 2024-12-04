pub fn p1(s: String) {
    let mut count = 0;
    let lines = s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>();

    for (y_index, l) in lines.clone().iter().enumerate() {
        count += l
            .as_bytes()
            .windows(4)
            .filter(|&w| (w == "XMAS".as_bytes()) || (w == "SAMX".as_bytes()))
            .count();

        if y_index >= 3 {
            for (c_index, c) in l.chars().enumerate() {
                if !(c_index <= (l.len() - 4)) {
                    continue;
                }
                let up_right: Vec<char> = vec![
                    c,
                    lines[y_index - 1].chars().collect::<Vec<char>>()[c_index + 1],
                    lines[y_index - 2].chars().collect::<Vec<char>>()[c_index + 2],
                    lines[y_index - 3].chars().collect::<Vec<char>>()[c_index + 3],
                ];
                if (up_right.iter().collect::<String>() == "XMAS".to_string()) || (up_right.iter().collect::<String>() == "SAMX".to_string()) {
                    count += 1;
                }
            }
        }
    }

    let mut result: Vec<String> = (0..lines.len()).map(|_| "".to_string()).collect::<Vec<String>>();
    for row in &lines {
        for (x, item) in row.clone().chars().enumerate() {
            result[x].push(item);
        }
    }
    result.reverse();

    for (y_index, l) in result.clone().iter().enumerate() {
        count += l
            .as_bytes()
            .windows(4)
            .filter(|&w| (w == "XMAS".as_bytes()) || (w == "SAMX".as_bytes()))
            .count();

        if y_index >= 3 {
            for (c_index, c) in l.chars().enumerate() {
                if !(c_index <= (l.len() - 4)) {
                    continue;
                }
                let up_right: Vec<char> = vec![
                    c,
                    result[y_index - 1].chars().collect::<Vec<char>>()[c_index + 1],
                    result[y_index - 2].chars().collect::<Vec<char>>()[c_index + 2],
                    result[y_index - 3].chars().collect::<Vec<char>>()[c_index + 3],
                ];
                if (up_right.iter().collect::<String>() == "XMAS".to_string()) || (up_right.iter().collect::<String>() == "SAMX".to_string()) {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
}

pub fn p2(s: String) {
    let mut count = 0;
    let lines = s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>();

    for (y_index, l) in lines.clone().iter().enumerate() {
        if (y_index >= 1) && (y_index <= (lines.len() - 2)) {
            for (c_index, c) in l.chars().enumerate() {
                if !((c_index >= 1) && (c_index <= (l.len() - 2))) {
                    continue;
                }
                if c != 'A' {
                    continue;
                }

                let c = vec![
                    lines[y_index - 1].chars().collect::<Vec<char>>()[c_index - 1],
                    lines[y_index - 1].chars().collect::<Vec<char>>()[c_index + 1],
                    lines[y_index + 1].chars().collect::<Vec<char>>()[c_index - 1],
                    lines[y_index + 1].chars().collect::<Vec<char>>()[c_index + 1],
                ];

                if !((c.iter().filter(|x| **x == 'M').count() == 2) && (c.iter().filter(|x| **x == 'S').count() == 2)) {
                    continue;
                }

                if (c[0] == c[1]) || (c[0] == c[2]) || (c[2] == c[3]) || (c[3] == c[1]) {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
}