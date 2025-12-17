pub fn p1(s: String) {
    //     let s = "123 328  51 64
    //  45 64  387 23
    //   6 98  215 314
    // *   +   *   +  "
    //         .to_string();

    let height = s.lines().count();
    let numbers = s
        .lines()
        .take(s.lines().count() - 1)
        .map(|l| {
            l.to_string()
                .split(" ")
                .filter(|i| !i.is_empty())
                .map(|i| i.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let numbers = (0..numbers[0].len())
        .map(move |x| {
            (0..height - 1)
                .map(|y| numbers[y][x].clone())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let operations = s
        .lines()
        .rev()
        .take(1)
        .map(|l| {
            l.split(" ")
                .filter(|c| !c.is_empty())
                .map(|c| c.chars().next().unwrap())
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()[0]
        .clone();

    println!(
        "total: {}",
        operations
            .iter()
            .enumerate()
            .fold(0, |sum, (index, val)| sum
                + (match val {
                    '*' => numbers[index].iter().fold(1, |prod, i| prod * i),
                    _ => numbers[index].iter().sum(),
                }))
    )
}

pub fn p2(s: String) {}
