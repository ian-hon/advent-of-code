mod challenge_input;
mod d9;

fn main() {
    // for i in 0..27 {
    //     println!("{}", i % 3);
    // }

    // let mut x = 30;
    // println!("{}", d7::format_radix(x, 3));

    d9::p2(challenge_input::input(9));
}
