mod bitboards;
mod movegen;

use bitboards::{print_bitboard, Color, Square};
use movegen::{bishop_attack_mask, bishop_mask, rook_mask, LookUp};
fn main() {
    let board = bitboards::Board::default();
    println!("{}", board);

    let lookup = LookUp::init();
    print_bitboard(&bishop_attack_mask(Square::E4 as usize, board.pieces[0]));
}
