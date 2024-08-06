mod bitboards;
mod movegen;

use bitboards::*;
use movegen::*;
fn main() {
    // let board = bitboards::Board::default();
    // println!("{}", board);
    //
    // let lookup = LookUp::init();
    // print_bitboard(&board.pieces[0]);
    // println!("{}", bitscan_forwards(board.pieces[0]))
    for _ in 0..1000 {
        println!(
            "{}",
            find_magic(Square::E4 as u64, ROOK_BITS[Square::E4 as usize], false)
        );
    }
}
