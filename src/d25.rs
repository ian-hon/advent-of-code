pub fn run() {
let data_ = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>".to_string();

    let mut map: Vec<Vec<i32>> = vec![];

    for (y, line) in data.split('\n').into_iter().map(|x| x.to_string()).collect::<Vec<String>>().into_iter().enumerate() {
        map.push(vec![]);
        for (x, c) in line.chars().enumerate() {
            map[y].push(match c { '>' => 1, 'v' => 2, _ => 0 });
        }
    }

    let w = map[0].len();
    let h = map.len();

    let mut count = 0;
    loop {
        println!("step : {count}");
        let mut flag = false;
        let mut next = map.clone();
        for (y, row) in map.clone().iter().enumerate() {
            for (x, e) in row.iter().enumerate() {
                if *e == 1 {
                    if get_at(x + 1, y, &map, w, h) == 0 {
                        next[y][x] = 0;
                        set_at(x + 1, y, &mut next, w, h, 1);
                        flag = true;
                    }
                }
            }
        }

        map = next.clone();

        for (y, row) in map.clone().iter().enumerate() {
            for (x, e) in row.iter().enumerate() {
                if *e == 2 {
                    if get_at(x, y + 1, &map, w, h) == 0 {
                        next[y][x] = 0;
                        set_at(x, y + 1, &mut next, w, h, 2);
                        flag = true;
                    }
                }
            }
        }

        count += 1;
        map = next;
        if !flag {
            break;
        }

        for row in &map {
            let mut result = "".to_string();
            for x in row {
                result += match x { 1 => ">", 2 => "v", _ => "." }
            }
        }
    }
    println!("steps : {count}");
}

fn get_at(x: usize, y: usize, m: &Vec<Vec<i32>>, w: usize, h: usize) -> i32 {
    let mut x = x;
    let mut y = y;
    if x >= w {
        x = x - w;
    }

    if y >= h {
        y = y - h;
    }

    m[y][x]
}

fn set_at(x: usize, y: usize, m: &mut Vec<Vec<i32>>, w: usize, h: usize, v : i32) {
    let mut x = x;
    let mut y = y;
    if x >= w {
        x = x - w;
    }

    if y >= h {
        y = y - h;
    }

    m[y][x] = v;
}
