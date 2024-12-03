pub fn p1(s: String) {
    let lines = s.split("mul(").map(|x| x.to_string()).collect::<Vec<String>>();

    let mut result = 0;

    for l in lines {
        let sec = l.split(")").map(|x| x.to_string()).collect::<Vec<String>>();
        if sec.is_empty() {
            continue;
        }

        let sec = sec[0].split(",").map(|x| x.to_string()).collect::<Vec<String>>();
        if sec.len() >= 3 {
            continue;
        }
        if sec.is_empty() {
            continue;
        }

        let first = sec[0].clone().parse::<i32>();
        if first.is_err() {
            continue;
        }
        let first = first.unwrap();

        let last = sec[1].clone().parse::<i32>();
        if last.is_err() {
            continue;
        }
        let last = last.unwrap();

        result += first * last;
    }

    println!("{result}");
}

pub fn p2(s: String) {
    // too lazy
    let lines = s.split("mul(").map(|x| x.to_string()).collect::<Vec<String>>();

    let mut result = 0;
    let mut dont = false;

    for l in lines {
        let sec = l.split(")").map(|x| x.to_string()).collect::<Vec<String>>();
        if sec.is_empty() {
            continue;
        }

        let sec = sec[0].split(",").map(|x| x.to_string()).collect::<Vec<String>>();
        if sec.len() >= 3 {
            continue;
        }
        if sec.is_empty() {
            continue;
        }



        if dont {
            continue;
        }

        let first = sec[0].clone().parse::<i32>();
        if first.is_err() {
            continue;
        }
        let first = first.unwrap();
        
        let last = sec[1].clone().parse::<i32>();
        if last.is_err() {
            continue;
        }
        let last = last.unwrap();

        result += first * last;
    }

    println!("{result}");
}

// 181345830
