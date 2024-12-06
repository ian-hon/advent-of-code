mod challenge_input;
mod d6;

fn main() {
    // let mut a = 3;
    // // a = (a << 4) | 1;
    // a |= 1 << 3;
    // println!("{a}");

    d6::p2(challenge_input::input(6));
}
