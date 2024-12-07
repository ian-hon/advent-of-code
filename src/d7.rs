pub fn p1(s: String) {
    let lines = s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>();

    let mut count = 0;
    for l in lines {
        let target = l.split(":").map(|x| x.to_string()).collect::<Vec<String>>()[0].parse::<i64>().unwrap();
        let operands = l.split(":").map(|x| x.to_string()).collect::<Vec<String>>()[1].strip_prefix(" ").unwrap().split(" ").map(|x| x.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>();

        // + or *

        for i in 0..2i64.pow(operands.len() as u32 - 1) {
            let mut total = operands[0];
            // let mut previous = operands[0];
            for t in operands.get(1..(operands.len())).unwrap().to_vec().iter().enumerate() {
                if ((i >> t.0 as i32) & 1) == 1 {
                    total += t.1;
                } else {
                    total *= t.1;
                }
            }
            if total == target {
                count += target;
                break;
            }
        }
    }
    println!("{count}");
}

pub fn p2(s: String) {
    let lines = s.split('\n').map(|x| x.to_string()).collect::<Vec<String>>();

    let mut count = 0;
    for (l_index, l) in lines.iter().enumerate() {
        println!("{l_index}/{}: {l}", lines.len());
        let target = l.split(":").map(|x| x.to_string()).collect::<Vec<String>>()[0].parse::<i64>().unwrap();
        let operands = l.split(":").map(|x| x.to_string()).collect::<Vec<String>>()[1].strip_prefix(" ").unwrap().split(" ").map(|x| x.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>();

        for i in 0..3i64.pow(operands.len() as u32 - 1) {
            let mut mask = format_radix(i as u32, 3).chars().map(|x| x.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>();
            let mut total = operands[0];
            while mask.len() < (operands.len() - 1) {
                mask.insert(0, 0);
            }
            for t in operands.get(1..(operands.len())).unwrap().to_vec().iter().enumerate() {
                if (mask[t.0]) == 0 {
                    total += t.1;
                } else if (mask[t.0]) == 1 {
                    total *= t.1;
                } else if (mask[t.0]) == 2 {
                    total = format!("{total}{}", t.1).parse::<i64>().unwrap();
                }
            }
            if total == target {
                count += target;
                break;
            }
        }
    }
    // 362646859298554
    println!("{count}");
}

fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}