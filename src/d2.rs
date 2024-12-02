pub fn p1(s: String) {
    let mut count = 0 ;
    for i in s.split("\n") {
        let n = i.to_string().split(" ").map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let mut prev = n[0];
        let mut flag = true;
        let direction = (n[0] - n[1]).signum();
        for ii in 1..n.len() {
            let c = n[ii];
            if ((prev - c).abs() >= 4) || (prev == c) || ((prev - c).signum() != direction) {
                flag = false;
                break;
            }
            prev = c;
        }
        if flag {
            println!("{i}");
            count += 1;
        }
    }

    println!("{count}");
}

pub fn p2(s: String) {
    
    let mut count = 0 ;
    for i in s.split("\n") {
        let n = i.to_string().split(" ").map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();

        if safe(n.clone()) {
            count += 1;
            continue;
        }

        let mut flag = false;
        for index in 0..n.len() {
            let new_n = n.clone().iter().enumerate().filter(|(idx, _)| *idx != index).map(|e| *e.1).collect::<Vec<i32>>();

            if safe(new_n) {
                flag = true;
                break;
            }
        }
        if !flag {
            continue;
        }

        count += 1;
    }

    println!("{count}");
}

fn safe(n: Vec<i32>) -> bool {
    let mut prev = n[0];
    let direction = (n[0] - n[1]).signum();
    for ii in 1..n.len() {
        let c = n[ii];
        if ((prev - c).abs() >= 4) || (prev == c) || ((prev - c).signum() != direction) {
            return false
        }
        prev = c;
    }
    true
}
