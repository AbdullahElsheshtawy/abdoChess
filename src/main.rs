mod bitboards;
mod movegen;

use bitboards::print_bitboard;
use bitboards::{Color, Square};
use movegen::init_lookups;
fn main() {
    let board = bitboards::Board::default();
    println!("{}", board);

    let lookup = init_lookups();
    print_bitboard(&lookup.double_pawn_attacks[Color::Black as usize][Square::A7 as usize]);
}
