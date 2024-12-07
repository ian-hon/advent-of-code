mod challenge_input;
mod d7;

fn main() {
    // for i in 0..27 {
    //     println!("{}", i % 3);
    // }

    // let mut x = 30;
    // println!("{}", d7::format_radix(x, 3));

    d7::p1(challenge_input::input(7));
}
