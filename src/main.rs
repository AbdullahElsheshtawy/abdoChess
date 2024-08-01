use bitboards::{Piece, PieceType};
use movegen::{pawn_attacks, pawn_attacks_black};

mod bitboards;
mod movegen;
fn main() {
    let board = bitboards::Board::default();
    println!("{}", board);
    bitboards::print_bitboard(&movegen::white_double_pawn_push(
        board.pieces[0] & board.colors[0],
        !(board.colors[0] | board.colors[1]),
    ));

    println!();
    bitboards::print_bitboard(&pawn_attacks(
        board.pieces[0] & board.colors[0],
        board.colors[1],
    ));
    println!();
    // bitboards::print_bitboard(&movegen::black_double_pawn_push(
    //     board.pieces[0] & board.colors[1],
    //     !(board.colors[0] | board.colors[1]),
    // ));
    //
    // println!();
    // bitboards::print_bitboard(&movegen::pawn_attacks_black(
    //     board.pieces[0] & board.colors[1],
    //     board.colors[0],
    // ));
}
