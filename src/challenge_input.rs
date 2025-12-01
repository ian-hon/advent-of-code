pub fn input(day: i32) -> String {
    let n: Vec<&str> = vec![""];
    let mut x: Vec<String> = vec![];
    for i in n {
        x.push(i.to_string());
    }
    return x[(day - 1) as usize].to_owned();
}
