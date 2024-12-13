pub fn p1(s: String) {
    let s_ = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279".to_string();

    let rounds = s.split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>();

    let mut score = 0;

    for round in rounds {
        let r = round.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();

        let a = r[0].split(": ").map(|x| x.to_string()).last().unwrap().split(", ").map(|x| x.to_string()).collect::<Vec<String>>().into_iter().map(|x| x.split("+").last().unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let b = r[1].split(": ").map(|x| x.to_string()).last().unwrap().split(", ").map(|x| x.to_string()).collect::<Vec<String>>().into_iter().map(|x| x.split("+").last().unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let f = r[2].split(": ").map(|x| x.to_string()).last().unwrap().split(", ").map(|x| x.to_string()).collect::<Vec<String>>().into_iter().map(|x| x.split("=").last().unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let mut results: Vec<i32> = vec![];
        for n in 0..=100 {
            for m in 0..=100 {
                let candidate = vec![
                    (a[0] * n) + (b[0] * m),
                    (a[1] * n) + (b[1] * m)
                ];

                if candidate == f {
                    results.push(m + (n * 3));
                }
            }
        }

        results.sort();

        match results.first() {
            Some(first) => {
                score += first;
            },
            None => {}
        };
    }

    println!("{score}");
}

pub fn p2(s: String) {
    let s_ = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279".to_string();

    let rounds = s.split("\n\n").map(|x| x.to_string()).collect::<Vec<String>>();
    let threshold = 0.01;

    let mut score = 0;
    for round in rounds {
        let r = round.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();

        let first_row = r[0].split(": ").map(|x| x.to_string()).last().unwrap().split(", ").map(|x| x.to_string()).collect::<Vec<String>>().into_iter().map(|x| x.split("+").last().unwrap().parse::<i64>().unwrap() as f64).collect::<Vec<f64>>();
        let second_row = r[1].split(": ").map(|x| x.to_string()).last().unwrap().split(", ").map(|x| x.to_string()).collect::<Vec<String>>().into_iter().map(|x| x.split("+").last().unwrap().parse::<i64>().unwrap() as f64).collect::<Vec<f64>>();
        let target_values = r[2].split(": ").map(|x| x.to_string()).last().unwrap().split(", ").map(|x| x.to_string()).collect::<Vec<String>>().into_iter().map(|x| x.split("=").last().unwrap().parse::<i64>().unwrap() as f64 + 10000000000000f64).collect::<Vec<f64>>();

        let m = (target_values[1] - ((first_row[1] * target_values[0]) / first_row[0])) / (second_row[1] - ((first_row[1] * second_row[0]) / first_row[0]));
        let n = (target_values[0] - (m * second_row[0])) / first_row[0];


        let m_i = m.round() as i64;
        let n_i = n.round() as i64;

        if (((m_i as f64) - m).abs() <= threshold) && (((n_i as f64) - n).abs() <= threshold) {
            score += m_i + (n_i * 3);
        }
    }

    println!("{score}");
}