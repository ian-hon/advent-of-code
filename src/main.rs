use crate::days::{d1, d10, d11, d12, d2, d3, d4, d5, d6, d7, d8, d9};
use std::{env, fs};

mod days;

type DayType = fn(String);

fn days() -> Vec<Vec<DayType>> {
    vec![
        vec![d1::p1, d1::p2],
        vec![d2::p1, d2::p2],
        vec![d3::p1, d3::p2],
        vec![d4::p1, d4::p2],
        vec![d5::p1, d5::p2],
        vec![d6::p1, d6::p2],
        vec![d7::p1, d7::p2],
        vec![d8::p1, d8::p2],
        vec![d9::p1, d9::p2],
        vec![d10::p1, d10::p2],
        vec![d11::p1, d11::p2],
        vec![d12::p1, d12::p2],
    ]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let d = args
        .get(1)
        .expect("no day supplied")
        .parse::<usize>()
        .expect("not a day");
    let p = args.get(2);
    if let Some(p) = p {
        let p = p.parse::<usize>().expect("not a part");
        days()[d - 1][p - 1](get_input(d));
        return;
    }
    days()[d - 1][0](get_input(d));
}

fn get_input(day: usize) -> String {
    fs::read_to_string(format!("inputs/D{day}")).unwrap_or_else(|_| download_input(day))
}

fn download_input(day: usize) -> String {
    let session = fs::read_to_string(".session").unwrap();
    let client = reqwest::blocking::Client::new();

    let url = format!("https://adventofcode.com/2025/day/{}/input", day);
    let response = client
        .get(url.clone())
        .header("cookie", format!("session={};", session))
        .send()
        .unwrap();

    if response.status().is_success() {
        let mut r = response.text().unwrap();
        r.pop();

        let path = format!("inputs/D{}", day);
        fs::write(&path, r.clone()).unwrap();
        return r;
    }

    panic!("err fetching")
}
