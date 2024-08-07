mod bitboards;
mod movegen;

use bitboards::*;
use movegen::*;
fn main() {
    let board = bitboards::Board::default();
    println!("{}", board);

    let lookup = LookUp::init();
    // for sq in 0..64 {
    //     for i in 0..64 {
    //         println!();
    print_bitboard(&lookup.bishop_attacks[Square::E4 as usize][1]);
    //         println!();
    //     }
    // }
}
