pub fn p1(s: String) {
//     let s = "0 3 6 9 12 15
// 1 3 6 10 15 21
// 10 13 16 21 30 45".to_string();
    let input_data = s.split('\n').map(|x| x.split(' ').map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let mut total = 0;
    for i in input_data {
        let mut d = get_all_differences(i);

        // let c = extrapolate_next(earlier, later)

        let length = d.len() - 1;
        for index in 0..(d.len() - 1) {
            let earlier_index = length - (index + 1);
            let later_index = length - index;

            let c = extrapolate_next(d[earlier_index].clone(), d[later_index].clone());

            d[earlier_index].push(c);
        }

        // println!("d: {d:?}");
        let extrapolated_value = d[0].last().unwrap();
        total += extrapolated_value;

        // println!("extrapolated value : {extrapolated_value}");
    }

    println!("total : {total}");
}

pub fn p2(s: String) {
//     let s = "0 3 6 9 12 15
// 1 3 6 10 15 21
// 10 13 16 21 30 45".to_string();
    let input_data = s.split('\n').map(|x| x.split(' ').map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();
    let mut total = 0;
    for i in input_data {
        let mut d = get_all_differences(i);

        let length = d.len() - 1;
        for index in 0..(d.len() - 1) {
            let earlier_index = length - (index + 1);
            let later_index = length - index;

            let c = extrapolate_previous(d[earlier_index].clone(), d[later_index].clone());

            d[earlier_index].insert(0, c);
        }

        let extrapolated_value = d[0].first().unwrap();
        total += extrapolated_value;

        println!("extrapolated value : {extrapolated_value}");
    }

    println!("total : {total}");
}

// 10  13  16  21  30  45  68
//    3   3   5   9  15  23
//      0   2   4   6   8
//        2   2   2a  2c
//          0   0   0b

// 10  13  16  21  30  45  68
//    3   3   5   9  15  23
//      0   2   4   6a  8c
//        2   2   2   2b
//          0   0   0

// 10  13  16  21  30  45  68
//    3   3   5   9  15a 23c
//      0   2   4   6   8b
//        2   2   2   2
//          0   0   0

// 10  13  16  21  30  45a 68c
//    3   3   5   9  15  23b
//      0   2   4   6   8
//        2   2   2   2
//          0   0   0

//                      c = a + b

// 5c 10a 13  16  21  30  45
//   5b  3   3   5   9  15
//    -2   0   2   4   6
//       2   2   2   2
//         0   0   0

// 5  10  13  16  21  30  45
//   5c  3a  3   5   9  15
//    -2b  0   2   4   6
//       2   2   2   2
//         0   0   0

// 5  10  13  16  21  30  45
//   5   3   3   5   9  15
//    -2c  0a  2   4   6
//       2b  2   2   2
//         0   0   0

// 5  10  13  16  21  30  45
//   5   3   3   5   9  15
//    -2   0   2   4   6
//       2c  2a  2   2
//         0b  0   0

// c = a - b

fn extrapolate_previous(earlier: Vec<i32>, later: Vec<i32>) -> i32 {
    let a = *earlier.first().unwrap();
    let b = *later.first().unwrap();

    a - b
}

fn extrapolate_next(earlier: Vec<i32>, later: Vec<i32>) -> i32 {
    let a = *earlier.last().unwrap();
    let b = *later.last().unwrap();

    a + b
}

fn get_all_differences(v: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![v.clone()];

    loop {
        let current = calculate_differences(result.last().unwrap());
        result.push(current.clone());

        if zero_vector(&current) {
            return result;
        }
    }
}

fn zero_vector(v: &Vec<i32>) -> bool {
    for i in v {
        if *i != 0 {
            return false;
        }
    }
    true
}

fn calculate_differences(data: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    let mut previous: Option<i32> = None;

    for d in data {
        if previous.is_none() {
            previous = Some(d.clone());
            continue;
        }

        let i = d - previous.unwrap();

        result.push(i.clone());
        previous = Some(d.clone());
    }

    result
}