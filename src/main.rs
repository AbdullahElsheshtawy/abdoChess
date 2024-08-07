mod bitboards;
mod movegen;

use bitboards::*;
use movegen::*;
fn main() {
    let board = bitboards::Board::default();
    println!("{}", board);

    let lookup = LookUp::init();
}
