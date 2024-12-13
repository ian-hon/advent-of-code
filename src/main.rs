mod challenge_input;
mod d13;

fn main() {
    // for i in 0..27 {
    //     println!("{}", i % 3);
    // }

    // let mut x = 30;
    // println!("{}", d7::format_radix(x, 3));


    d13::p2(challenge_input::input(13));
}
