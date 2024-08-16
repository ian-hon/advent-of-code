pub fn p1(s: String) {
//     let s = "???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1";
    // 0., 1#, 2?

    println!("{s}");
    let mut field: Vec<(Vec<u8>, Vec<i32>)> = vec![];
    for raw_row in s.split("\n") {
        let row = raw_row.split(' ').collect::<Vec<&str>>()[0];

        field.push((
            row.chars().map(|c| match c {
                '.' => 0,
                '#' => 1,
                '?' => 2,
                _ => 3
            })
            .collect::<Vec<u8>>(),
            raw_row
                .split(' ')
                .collect::<Vec<&str>>()[1]
                .to_string()
                .chars()
                .filter(|x| *x != ',')
                .map(|x| x.to_string().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
            )
        );
    }

    let mut result = 0;

    for (row_index, row) in field.iter().enumerate() {
        println!("row number : {row_index}");
        let slots: Vec<usize> = row.0.clone().iter().enumerate().filter(|x| *x.1 == 2).map(|x| x.0).collect::<Vec<usize>>();

        for index in 0..(2i32.pow(slots.len() as u32)) {
            let latch_system: Vec<bool> = (0..(slots.len())).map(|x| ((index >> x) & 1) == 1).collect();

            let mut r = row.0.clone();

            let mut counter = 0;
            for i in &mut r {
                if *i == 2 {
                    *i = if latch_system[counter as usize] { 1 } else { 0 };

                    counter += 1;
                }
            }

            if parse(r.clone()) == row.1 {
                println!("{} : {:?}", translate(r.clone()), parse(r.clone()));
                result += 1;
            }
        }
    }

    // 7098 (too low)

    println!("Result : {result}");
}

fn parse(collection: Vec<u8>) -> Vec<i32> {
    let mut result = vec![];

    let mut count = 0;

    for i in &collection {
        if *i == 1 {
            count += 1;
        } else {
            if count == 0 {
                continue;
            }
            result.push(count);
            count = 0;
        }
    }
    if count != 0 {
        result.push(count);
    }

    result
}

fn translate(c: Vec<u8>) -> String {
    c.iter().map(|x| match x {
        0 => '.',
        1 => '#',
        2 => '?',
        _ => '!'
    })
    .collect()
}
